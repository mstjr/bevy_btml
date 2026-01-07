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

    let items = vec!["Item 1", "Item 2", "Item 3"];

    btml!(commands,
        <Node
            width=Val::Percent(100.0),
            height=Val::Percent(100.0),
            flex_direction=FlexDirection::Column,
            align_items=AlignItems::Center,
            justify_content=JustifyContent::Center,
            row_gap=Val::Px(10.0)
        >
            <children>
                <Text>"List of Items:".to_string()</Text>
                <TextFont font_size=30.0 />
                <TextColor>Color::WHITE</TextColor>

                //All element will be seperated children of the parent since it can't have twice the same parent
                for item in items.iter() {
                    <Node
                        padding=UiRect::all(Val::Px(10.0)),
                        border=UiRect::all(Val::Px(2.0))
                    >
                        <BorderColor(all)>Color::WHITE</BorderColor>
                        <BackgroundColor>Color::srgb(0.2, 0.2, 0.2)</BackgroundColor>
                        <children>
                            <Text>format!("- {}", item)</Text>
                            <TextFont font_size=20.0 />
                            <TextColor>Color::srgb(0.9, 0.9, 0.9)</TextColor>
                        </children>
                    </Node>
                }
            </children>
        </Node>
    );
}
