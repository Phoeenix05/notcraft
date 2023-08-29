use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3 {
                x: 10.0,
                y: 8.0,
                z: -12.5,
            },
            ..default()
        },
        ..default()
    });
}
