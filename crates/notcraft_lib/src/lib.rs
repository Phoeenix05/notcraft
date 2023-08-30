use bevy::prelude::*;

#[derive(Component)]
pub struct Rotator;

/// Rotate the cube to make it clear when the app is updating
pub fn rotate_cube(time: Res<Time>, mut cube_transform: Query<&mut Transform, With<Rotator>>) {
    for mut transform in &mut cube_transform {
        transform.rotate_x(time.delta_seconds());
        transform.rotate_local_y(time.delta_seconds());
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..default()
        },
        Rotator,
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
            ..default()
        },
        ..default()
    });
}
