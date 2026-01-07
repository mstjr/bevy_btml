use bevy::prelude::*;
use bevy_btml::btml;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let show_button = true;
    let condition_val = 2;

    btml!(commands,
        <Node width=Val::Percent(100.0), height=Val::Percent(100.0), flex_direction=FlexDirection::Column, justify_content=JustifyContent::Center, align_items=AlignItems::Center>
            <children>
                <Text>"Conditional Rendering".to_string()</Text>
                <TextFont font_size=30.0 />
                <TextColor>Color::WHITE</TextColor>

                if show_button {
                    <Node padding=UiRect::all(Val::Px(10.0))>
                         <children>
                            <Text>"Button Shown".to_string()</Text>
                            <TextFont font_size=20.0 />
                            <TextColor>Color::WHITE</TextColor>
                         </children>
                    </Node>
                } else {
                    <Node default>
                         <children>
                            <Text>"Button Hidden".to_string()</Text>
                            <TextFont font_size=20.0 />
                            <TextColor>Color::WHITE</TextColor>
                         </children>
                    </Node>
                }

                // Else If Test
                if condition_val == 1 {
                    <Text>"Condition is 1".to_string()</Text>
                    <TextFont font_size=25.0 />
                    <TextColor>Color::WHITE</TextColor>
                } else if condition_val == 2 {
                    <Text>"Condition is 2".to_string()</Text>
                    <TextFont font_size=25.0 />
                    <TextColor>Color::srgb(0.0, 1.0, 0.0)</TextColor>
                } else {
                    <Text>"Condition is Something Else".to_string()</Text>
                    <TextFont font_size=25.0 />
                    <TextColor>Color::WHITE</TextColor>
                }
            </children>
        </Node>
    );
}
