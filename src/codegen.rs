use crate::parse::{BtmlChild, BtmlElse, BtmlNode, Content};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_bundle_code(nodes: &[BtmlChild]) -> TokenStream {
    let mut components = Vec::new();

    for child in nodes {
        match child {
            BtmlChild::Node(node) => {
                if node.tag == "children" {
                    continue;
                }
                components.push(node_to_component(node));
                collect_components_recursive(node, &mut components);
            }
            BtmlChild::For(_) | BtmlChild::If(_) => {
                return quote! { compile_error!("Control flow (For/If) is not allowed in bundle-only mode. Use a spawner.") };
            }
        }
    }

    quote! {
        (
            #( #components ),*
        )
    }
}

fn collect_components_recursive(node: &BtmlNode, components: &mut Vec<TokenStream>) {
    for child in &node.children {
        match child {
            BtmlChild::Node(child_node) => {
                if child_node.tag != "children" {
                    components.push(node_to_component(child_node));
                    collect_components_recursive(child_node, components);
                }
            }
            BtmlChild::For(_) | BtmlChild::If(_) => {}
        }
    }
}

pub fn generate_spawn_code(spawner: &Ident, nodes: &[BtmlChild]) -> TokenStream {
    let mut components = Vec::new();
    let mut children_generators = Vec::new();

    for child in nodes {
        collect_components_and_children(child, &mut components, &mut children_generators);
    }

    let spawn_expr = quote! {
        #spawner.spawn((
            #( #components ),*
        ))
    };

    if children_generators.is_empty() {
        spawn_expr
    } else {
        let mut child_spawns = Vec::new();

        for generator in children_generators {
            match generator {
                BtmlChild::Node(child_block) => {
                    let new_spawner = Ident::new("parent", proc_macro2::Span::call_site());
                    let child_code = generate_spawn_code(&new_spawner, &child_block.children);
                    child_spawns.push(child_code);
                }
                BtmlChild::For(for_loop) => {
                    let pat = &for_loop.pat;
                    let expr = &for_loop.expr;
                    let body = &for_loop.body;

                    let new_spawner = Ident::new("parent", proc_macro2::Span::call_site());
                    let body_code = generate_spawn_code(&new_spawner, body);

                    let loop_code = quote! {
                        for #pat in #expr {
                            #body_code;
                        }
                    };
                    child_spawns.push(loop_code);
                }
                BtmlChild::If(if_node) => {
                    let new_spawner = Ident::new("parent", proc_macro2::Span::call_site());
                    let if_code = generate_if_code(&new_spawner, if_node);
                    child_spawns.push(if_code);
                }
            }
        }

        quote! {
            #spawn_expr.with_children(|parent| {
                #( #child_spawns; )*
            })
        }
    }
}

fn generate_if_code(spawner: &Ident, if_node: &crate::parse::BtmlIf) -> TokenStream {
    let condition = &if_node.condition;
    let then_code = generate_spawn_code(spawner, &if_node.then_branch);

    match &if_node.else_branch {
        None => {
            quote! {
                if #condition {
                    #then_code;
                }
            }
        }
        Some(else_branch) => {
            let else_code = match &**else_branch {
                BtmlElse::Block(block) => {
                    let block_code = generate_spawn_code(spawner, block);
                    quote! { #block_code; }
                }
                BtmlElse::If(else_if) => generate_if_code(spawner, else_if),
            };

            quote! {
                if #condition {
                    #then_code;
                } else {
                    #else_code
                }
            }
        }
    }
}

fn collect_components_and_children<'a>(
    child: &'a BtmlChild,
    components: &mut Vec<TokenStream>,
    children_generators: &mut Vec<&'a BtmlChild>,
) {
    match child {
        BtmlChild::Node(node) => {
            if node.tag == "children" {
                children_generators.push(child);
            } else {
                components.push(node_to_component(node));
                for inner_child in &node.children {
                    collect_components_and_children(inner_child, components, children_generators);
                }
            }
        }
        BtmlChild::For(_) | BtmlChild::If(_) => {
            children_generators.push(child);
        }
    }
}

fn node_to_component(node: &BtmlNode) -> TokenStream {
    let name = &node.tag;

    let has_default = node.flags.iter().any(|f| *f == "default");
    let has_no_default = node.flags.iter().any(|f| *f == "no_default");

    if has_no_default && has_default {
        panic!("Cannot use both 'default' and 'no_default' attributes on the same component.");
    }

    if let Some(constructor) = &node.constructor {
        match &node.content {
            Some(Content::Arguments(args)) => {
                quote! { #name::#constructor(#args) }
            }
            None => {
                quote! { #name::#constructor() }
            }
        }
    } else if let Some(content) = &node.content {
        match content {
            Content::Arguments(args) => {
                quote! { #name(#args) }
            }
        }
    } else if !node.attributes.is_empty() {
        let attrs = node.attributes.iter().map(|a| {
            let key = &a.key;
            let val = &a.value;
            quote! { #key: #val }
        });

        if has_no_default {
            quote! {
                #name {
                    #( #attrs ),*
                }
            }
        } else {
            quote! {
                #name {
                    #( #attrs ),*,
                    ..Default::default()
                }
            }
        }
    } else if has_default {
        quote! { #name::default() }
    } else {
        quote! {
            #name
        }
    }
}
