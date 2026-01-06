use bevy::prelude::*;
use bevy_btml::btml;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let image: Handle<Image> = asset_server.load("coconuts.png");

    btml!(commands,
        <Node display=Display::Flex, flex_direction=FlexDirection::Column, align_items=AlignItems::Center, justify_content=JustifyContent::Center, width=Val::Percent(100.0), height=Val::Percent(100.0)>
            <BackgroundColor>Color::BLACK</BackgroundColor>
            <children>
                <ImageNode image=image.clone() />
            </children>
        </Node>
    );
}
