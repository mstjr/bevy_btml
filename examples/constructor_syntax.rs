use bevy::prelude::*;
use bevy_btml::btml;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    btml!(commands,
        <Node position_type=PositionType::Absolute, top=Val::Percent(50.), left=Val::Percent(50.)/>
        
        // Single-arg
        <Text(new)>"Child Text"</Text>

        // Multi-arg
        <UiTransform rotation=Rot2::from_sin_cos(1., 0.), translation=Val2::percent(-50., -50.) />
    );
}