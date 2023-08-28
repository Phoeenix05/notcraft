use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
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
        .add_systems(Startup, setup)
        .run();
    log::info!("test");
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
