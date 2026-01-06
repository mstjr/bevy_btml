use proc_macro2::TokenTree;
use syn::{
    Expr, Ident, Result, Token,
    parse::{Parse, ParseStream},
};

pub struct BtmlInput {
    pub spawner: Option<Ident>,
    pub nodes: Vec<BtmlNode>,
}

pub struct BtmlNode {
    pub tag: Ident,
    pub attributes: Vec<BtmlAttr>,
    pub flags: Vec<Ident>,
    pub children: Vec<BtmlNode>,
    pub content: Option<Content>,
}

#[derive(Debug)]
pub enum Content {
    Expr(Expr),
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
            let node: BtmlNode = input.parse()?;
            nodes.push(node);
        }

        Ok(BtmlInput { spawner, nodes })
    }
}

impl Parse for BtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let _lt: Token![<] = input.parse()?;
        let tag: Ident = input.parse()?;

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
            if !input.peek(Token![<]) {
                let mut tokens = proc_macro2::TokenStream::new();
                let mut text_acc = String::new();

                while !input.peek(Token![<]) && !input.is_empty() {
                    let tt: TokenTree = input.parse()?;
                    text_acc.push_str(&tt.to_string());
                    text_acc.push(' ');
                    tokens.extend(std::iter::once(tt));
                }

                if let Ok(expr) = syn::parse2::<Expr>(tokens.clone()) {
                    content = Some(Content::Expr(expr));
                }
            }

            while input.peek(Token![<]) && !input.peek2(Token![/]) {
                let child: BtmlNode = input.parse()?;
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
            attributes,
            flags,
            children,
            content,
        })
    }
}
