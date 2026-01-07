//! # Bevy Tree Markup Language
//!
//! A macro to create Bevy entity-component hierarchies using an HTML-like syntax.
//!
//! Even though it was made for UI, it can be used to define any entity-component hierarchy.
//!
//! ## `btml!` Macro
//!
//! The `btml!` macro allows you to define entities and their components in a concise and readable way.
//!
//! ### Syntax
//!
//! - **Components**: Initialize components using HTML-like tags. Attribute names correspond to struct fields.
//!   ```rust
//!   <Node width=Val::Percent(100.0) height=Val::Percent(100.0) />
//!   ```
//!
//! - **Defaults**:
//!   - `<Tag />`: Generates `Tag` as a value. Use this for Unit Structs (marker components) that don't need initialization.
//!   - `<Tag default />`: Generates `Tag::default()`. Use this for Structs with fields where you want the default values.
//!   - `<Tag attr=val />`: Generates `Tag { attr: val, ..Default::default() }`.
//!   - `<Tag flag, attr=val />`: Attributes without values (flags) are ignored during struct initialization but are used during macro logic (like `default` and `no_default` that tells the macro how to generate the component).
//!
//! - **Tuple Structs**: Initialize tuple structs (like `BackgroundColor` or `TextColor`) by providing the value as content.
//!   ```rust
//!   <BackgroundColor>Color::BLACK</BackgroundColor>
//!   ```
//!
//! - **Hierarchy**: Use the `<children>` tag to nest entities. Nested node without the `<children>` tag are treated as sibling components of the same entity, even if they are nested.
//!   ```rust
//!   btml!(commands,
//!       <Node default>
//!          <children>
//!             <Text(new)>"Child"</Text>
//!             <TextFont font_size=24.0 />
//!          </children>
//!       </Node>
//!   );
//!   //is the same as
//!   commands.spawn((
//!      Node::default(),
//!   )).with_children(|parent| {
//!      parent.spawn((Text::new("Child"), TextFont { font_size: 24.0, ..Default::default() }));
//!   });
//!   ```
//!
//! - **Control Flow**: Use Rust `for` loops inside `<children>` to dynamically create entities.
//!   ```rust
//!   <children>
//!       for item in items {
//!           <Text(new)>item.to_string()</Text>
//!       }
//!   </children>
//!   ```
//!   Note: All entities spawned within the loop are separate children of the parent entity. The loop itself does not create an intermediate wrapper entity.
//!
//! - **Conditional Rendering**: Use Rust `if` and `else` blocks to conditionally spawn entities.
//!   ```rust
//!   <children>
//!       if show_button {
//!           <Button(new)>"Click Me"</Button>
//!       } else {
//!           <Text(new)>"Button Hidden"</Text>
//!       }
//!   </children>
//!   ```
//!
//!
//! ### Example
//!
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_btml::btml;
//! # fn setup(mut commands: Commands) {
//! btml!(commands,
//!     <Node display=Display::Flex, justify_content=JustifyContent::Center>
//!         <BackgroundColor>Color::BLACK</BackgroundColor>
//!         <children>
//!             <Text(new)>"Hello World"</Text>
//!             <TextFont font_size=30.0 />
//!             <TextColor>Color::WHITE</TextColor>
//!         </children>
//!     </Node>
//! );
//! # }
//! ```
//!
//! # Other examples
//! You can find examples in the `examples/` folder of the repository.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod codegen;
mod parse;

use parse::BtmlInput;

#[proc_macro]
pub fn btml(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as BtmlInput);

    if let Some(spawner) = input_ast.spawner {
        codegen::generate_spawn_code(&spawner, &input_ast.nodes).into()
    } else {
        codegen::generate_bundle_code(&input_ast.nodes).into()
    }
}
