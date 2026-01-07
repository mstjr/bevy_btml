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
        <UiTransform(from_rotation)>
            Rot2 { cos: 0.0, sin: 1.0 }
        </UiTransform>
    );
}