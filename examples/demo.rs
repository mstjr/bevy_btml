use bevy::prelude::*;
use bevy_btml::btml;

const DARK_GRAY: Color = Color::srgb(0.2, 0.2, 0.2);
const GRAY: Color = Color::srgb(0.5, 0.5, 0.5);
const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

#[derive(Component)]
struct ClickMeButton;

#[derive(Component)]
struct Counter {
    counter: i32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    btml!(commands,
        <Node display=Display::Flex, flex_direction=FlexDirection::Column, align_items=AlignItems::Center, justify_content=JustifyContent::Center, width=Val::Percent(100.0), height=Val::Percent(100.0)>
            <BackgroundColor>Color::BLACK</BackgroundColor>
            <children>
                <Text>"Hello Bevy!".to_string()</Text>
                <TextFont font_size=30.0 />
                <TextColor>Color::WHITE</TextColor>
            </children>   
            <children>
                <Button />
                <ClickMeButton />
                <BackgroundColor>Color::BLACK</BackgroundColor>
                <children>
                    <Text>"Click Me!".to_string()</Text>
                    <TextFont font_size=20.0 />
                    <TextColor>Color::WHITE</TextColor>
                </children>
            </children>
            <children>
                <Text>"Counter: 0".to_string()</Text>
                <TextFont font_size=25.0 />
                <TextColor>YELLOW</TextColor>
                //You must use no_default for type with no Default implementation
                <Counter no_default counter=0/>
            </children>
        </Node>
    );
}

fn button_system(
    mut interaction: Single<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ClickMeButton>),
    >,
    mut counter_query: Single<(&mut Text, &mut Counter)>,
) {
    match interaction.0 {
        Interaction::Pressed => {
            counter_query.1.counter += 1;
            *counter_query.0 = format!("Counter: {}", counter_query.1.counter).into();
            interaction.1.0 = GRAY;
        }
        Interaction::Hovered => {
            interaction.1.0 = DARK_GRAY;
        }
        Interaction::None => {
            interaction.1.0 = Color::BLACK;
        }
    }
}
