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

    btml!(commands,
        <Node
            width=Val::Percent(100.0),
            height=Val::Percent(100.0),
            justify_content=JustifyContent::Center,
            align_items=AlignItems::Center
        >
             <children>
                <Text>"Game View".to_string()</Text>
                <TextFont font_size=50.0 />
             </children>
        </Node>
    );

    btml!(commands,
        <Node
            width=Val::Px(200.0),
            height=Val::Px(200.0),
            position_type=PositionType::Absolute,
            right=Val::Px(20.0),
            top=Val::Px(20.0),
            border=UiRect::all(Val::Px(2.0)),
        >
            <BackgroundColor>Color::BLACK</BackgroundColor>
            <BorderColor
                top=Color::WHITE,
                bottom=Color::WHITE,
                left=Color::WHITE,
                right=Color::WHITE
            />

            <children>
                <Node
                    width=Val::Px(10.0),
                    height=Val::Px(10.0),
                    position_type=PositionType::Absolute,
                    left=Val::Px(95.0),
                    top=Val::Px(95.0),
                >
                     <BackgroundColor>Color::srgb(0.0, 1.0, 0.0)</BackgroundColor>
                     <BorderRadius
                        top_left=Val::Percent(50.0),
                        top_right=Val::Percent(50.0),
                        bottom_left=Val::Percent(50.0),
                        bottom_right=Val::Percent(50.0)
                     />
                </Node>
            </children>
            <children>
                <Node
                    width=Val::Px(8.0),
                    height=Val::Px(8.0),
                    position_type=PositionType::Absolute,
                    left=Val::Px(40.0),
                    top=Val::Px(30.0),
                >
                     <BackgroundColor>Color::srgb(1.0, 0.0, 0.0)</BackgroundColor>
                     <BorderRadius
                        top_left=Val::Percent(50.0),
                        top_right=Val::Percent(50.0),
                        bottom_left=Val::Percent(50.0),
                        bottom_right=Val::Percent(50.0)
                     />
                </Node>
            </children>
            <children>
                <Node
                    width=Val::Px(30.0),
                    height=Val::Px(20.0),
                    position_type=PositionType::Absolute,
                    right=Val::Px(40.0),
                    bottom=Val::Px(50.0),
                >
                     <BackgroundColor>Color::srgb(0.5, 0.5, 1.0)</BackgroundColor>
                </Node>
            </children>
        </Node>
    );
}
