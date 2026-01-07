use crate::parse::{BtmlNode, Content};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_bundle_code(nodes: &[BtmlNode]) -> TokenStream {
    let mut components = Vec::new();

    for node in nodes {
        if node.tag == "children" {
            continue;
        }
        components.push(node_to_component(node));
        for child in &node.children {
            if child.tag != "children" {
                components.push(node_to_component(child));
            }
        }
    }

    quote! {
        (
            #( #components ),*
        )
    }
}

pub fn generate_spawn_code(spawner: &Ident, nodes: &[BtmlNode]) -> TokenStream {
    let mut components = Vec::new();
    let mut children_blocks = Vec::new();

    for node in nodes {
        collect_components_and_children(node, &mut components, &mut children_blocks);
    }

    let spawn_expr = quote! {
        #spawner.spawn((
            #( #components ),*
        ))
    };

    if children_blocks.is_empty() {
        spawn_expr
    } else {
        let mut child_spawns = Vec::new();
        for child_block in children_blocks {
            let new_spawner = Ident::new("parent", proc_macro2::Span::call_site());
            let child_code = generate_spawn_code(&new_spawner, &child_block.children);
            child_spawns.push(child_code);
        }

        quote! {
            #spawn_expr.with_children(|parent| {
                #( #child_spawns; )*
            })
        }
    }
}

fn collect_components_and_children<'a>(
    node: &'a BtmlNode,
    components: &mut Vec<TokenStream>,
    children_blocks: &mut Vec<&'a BtmlNode>,
) {
    if node.tag == "children" {
        children_blocks.push(node);
    } else {
        components.push(node_to_component(node));
        for child in &node.children {
            collect_components_and_children(child, components, children_blocks);
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
        // Case: <Tag(method)>...
        match &node.content {
            Some(Content::Arguments(args)) => {
                // <Tag(method)>arg1, arg2</Tag> -> Tag::method(arg1, arg2)
                quote! { #name::#constructor(#args) }
            },
            None => {
                // <Tag(method) /> -> Tag::method()
                quote! { #name::#constructor() }
            }
        }
    } else if let Some(content) = &node.content {
        // Case: <Tag>arg1, arg2</Tag> -> Tag(arg1, arg2)
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
