use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowTheme},
};

fn main() {
    env_logger::init();

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "NotCraft".into(),
                        present_mode: PresentMode::Immediate,
                        resolution: (1280.0, 800.0).into(),
                        resizable: false,
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        .add_systems(Startup, notcraft_lib::setup)
        .add_systems(Update, change_clear_color)
        .run();
    log::info!("test");
}

fn change_clear_color(
    mut clear_color: ResMut<ClearColor>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(pos) = q_windows.single().cursor_position() {
        clear_color.0 = Color::rgb(
            pos.x / 1280.0,
            (pos.x / 1280.0 / 2.0) + (pos.y / 800.0 / 2.0),
            pos.y / 800.0,
        );
    }
}
