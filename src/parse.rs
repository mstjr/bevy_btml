use proc_macro2::TokenTree;
use syn::{
    Expr, Ident, Pat, Result, Token, braced, parenthesized,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
};

pub struct BtmlInput {
    pub spawner: Option<Ident>,
    pub nodes: Vec<BtmlChild>,
}

pub enum BtmlChild {
    Node(BtmlNode),
    For(BtmlFor),
    If(BtmlIf),
}

pub struct BtmlFor {
    pub pat: Pat,
    pub expr: Expr,
    pub body: Vec<BtmlChild>,
}

pub struct BtmlIf {
    pub condition: Expr,
    pub then_branch: Vec<BtmlChild>,
    pub else_branch: Option<Box<BtmlElse>>,
}

pub enum BtmlElse {
    If(BtmlIf),
    Block(Vec<BtmlChild>),
}

pub struct BtmlNode {
    pub tag: Ident,
    pub constructor: Option<Ident>,
    pub attributes: Vec<BtmlAttr>,
    pub flags: Vec<Ident>,
    pub children: Vec<BtmlChild>,
    pub content: Option<Content>,
}

#[derive(Debug)]
pub enum Content {
    Arguments(Punctuated<Expr, Token![,]>),
}

#[derive(Debug)]
pub struct BtmlAttr {
    pub key: Ident,
    pub value: Expr,
}

impl Parse for BtmlInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let spawner = if input.peek(Ident) && input.peek2(Token![,]) {
            let id: Ident = input.parse()?;
            let _comma: Token![,] = input.parse()?;
            Some(id)
        } else {
            None
        };

        let mut nodes = Vec::new();
        while !input.is_empty() {
            let child: BtmlChild = input.parse()?;
            nodes.push(child);
        }

        Ok(BtmlInput { spawner, nodes })
    }
}

impl Parse for BtmlChild {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![for]) {
            let _for: Token![for] = input.parse()?;
            let pat: Pat = Pat::parse_multi_with_leading_vert(input)?;
            let _in: Token![in] = input.parse()?;
            let expr: Expr = Expr::parse_without_eager_brace(input)?;

            let content;
            braced!(content in input);

            let mut body = Vec::new();
            while !content.is_empty() {
                body.push(content.parse()?);
            }

            Ok(BtmlChild::For(BtmlFor { pat, expr, body }))
        } else if input.peek(Token![if]) {
            let _if: Token![if] = input.parse()?;
            let condition: Expr = Expr::parse_without_eager_brace(input)?;

            let content;
            braced!(content in input);

            let mut then_branch = Vec::new();
            while !content.is_empty() {
                then_branch.push(content.parse()?);
            }

            let mut else_branch = None;
            if input.peek(Token![else]) {
                let _else: Token![else] = input.parse()?;
                if input.peek(Token![if]) {
                    let next_if_child: BtmlChild = input.parse()?;
                    if let BtmlChild::If(next_if) = next_if_child {
                        else_branch = Some(Box::new(BtmlElse::If(next_if)));
                    } else {
                        return Err(input.error("Expected if after else"));
                    }
                } else {
                    let content;
                    braced!(content in input);
                    let mut else_body = Vec::new();
                    while !content.is_empty() {
                        else_body.push(content.parse()?);
                    }
                    else_branch = Some(Box::new(BtmlElse::Block(else_body)));
                }
            }

            Ok(BtmlChild::If(BtmlIf {
                condition,
                then_branch,
                else_branch,
            }))
        } else {
            let node: BtmlNode = input.parse()?;
            Ok(BtmlChild::Node(node))
        }
    }
}

impl Parse for BtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let _lt: Token![<] = input.parse()?;
        let tag: Ident = input.parse()?;

        let constructor = if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            Some(content.parse()?)
        } else {
            None
        };

        let mut attributes = Vec::new();
        let mut flags = Vec::new();

        while !input.peek(Token![>]) && !input.peek(Token![/]) {
            let key: Ident = input.parse()?;

            if input.peek(Token![=]) {
                let _eq: Token![=] = input.parse()?;

                let mut tokens = proc_macro2::TokenStream::new();
                while !input.is_empty() {
                    if input.peek(Token![,]) || input.peek(Token![>]) || input.peek(Token![/]) {
                        break;
                    }
                    let tt: TokenTree = input.parse()?;
                    tokens.extend(std::iter::once(tt));
                }

                if tokens.is_empty() {
                    return Err(input.error("Expected attribute value"));
                }
                let value = syn::parse2(tokens)?;
                attributes.push(BtmlAttr { key, value });
            } else {
                flags.push(key);
            }

            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            }
        }

        let mut is_self_closing = false;
        if input.peek(Token![/]) {
            let _slash: Token![/] = input.parse()?;
            let _gt: Token![>] = input.parse()?;
            is_self_closing = true;
        } else {
            let _gt: Token![>] = input.parse()?;
        }

        let mut children = Vec::new();
        let mut content = None;

        if !is_self_closing {
            if !input.peek(Token![<]) && !input.peek(Token![for]) && !input.peek(Token![if]) {
                let mut tokens = proc_macro2::TokenStream::new();

                while !input.peek(Token![<])
                    && !input.peek(Token![for])
                    && !input.peek(Token![if])
                    && !input.is_empty()
                {
                    let tt: TokenTree = input.parse()?;
                    tokens.extend(std::iter::once(tt));
                }

                if !tokens.is_empty() {
                    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
                    match parser.parse2(tokens) {
                        Ok(args) => {
                            content = Some(Content::Arguments(args));
                        }
                        Err(e) => return Err(e),
                    }
                }
            }

            while (input.peek(Token![<]) && !input.peek2(Token![/]))
                || input.peek(Token![for])
                || input.peek(Token![if])
            {
                let child: BtmlChild = input.parse()?;
                children.push(child);
            }

            let _lt: Token![<] = input.parse()?;
            let _slash: Token![/] = input.parse()?;
            let closing: Ident = input.parse()?;

            if closing != tag {
                return Err(syn::Error::new(
                    closing.span(),
                    format!("Expected closing tag </{}>, found </{}>", tag, closing),
                ));
            }
            let _gt: Token![>] = input.parse()?;
        }

        Ok(BtmlNode {
            tag,
            constructor,
            attributes,
            flags,
            children,
            content,
        })
    }
}
