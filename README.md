# Bevy Tree Markup Language (BTML)

A macro to create Bevy entity-component hierarchies using an HTML-like syntax.

Even though it was made for UI, it can be used to define any entity-component hierarchy.

## Features

- **`btml!` Macro**: Define your UI hierarchy using a familiar HTML-like structure.
- **Component Initialization**: Initialize components with named fields as attributes.
  ```rust
  <Node width=Val::Percent(100.0) height=Val::Percent(100.0) />
  ```
- **Tuple Struct Support**: Initialize tuple structs (like `BackgroundColor` or `TextColor`) using content syntax.
  ```rust
  <BackgroundColor>Color::BLACK</BackgroundColor>
  ```
- **Constructor Support**: Use method calling for components
  ```rust
  <Text(new)>"Hello World"</Text>
  ```
- **Hierarchy Support**: Nest children directly using the `<children>` tag.
  ```rust
  btml!(commands,
      <Node>
          <children>
              <Text(new)>"Child Text"</Text>
          </children>
      </Node>
  );
  ```
- **Expression Support**: Pass Rust expressions as attribute values or content.
- **Control Flow**: Use Rust `for` loops to dynamically create entities.
  ```rust
      for item in items {
          <Text(new)>item.to_string()</Text>
      }
  ```
  Note: All entities spawned within the loop are children of the parent entity. The loop itself does not create an intermediate wrapper entity.
- **Conditional Rendering**: Use Rust `if` and `else` blocks to conditionally spawn entities.
  ```rust
      if show_button {
          <Button(new)>"Click Me"</Button>
      } else {
          <Text(new)>"Button Hidden"</Text>
      }
  ```

## Usage

```rust
use bevy::prelude::*;
use bevy_btml::btml;

fn setup(mut commands: Commands) {
    btml!(commands,
        <Node
            width=Val::Percent(100.0),
            height=Val::Percent(100.0),
            justify_content=JustifyContent::Center,
            align_items=AlignItems::Center,
        >
            <BackgroundColor>Color::BLACK</BackgroundColor>
            <children>
                <Text(new)>"Hello Bevy!"</Text>
                <TextFont font_size=30.0 />
                <TextColor>Color::WHITE</TextColor>
            </children>
        </Node>
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
```

## Examples

You can find examples in the [examples/](./examples/) folder of the repository.

## Compatibility

| bevy | bevy_btml |
| ---- | --------- |
| 0.17 | `0.1.3`   |
