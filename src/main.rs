use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use rlua::Lua;

fn main() {
    env_logger::init();

    Lua::new().context(|ctx| {
        let globals = ctx.globals();

        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        let add_nums = ctx
            .create_function(|_, (a, b): (i32, i32)| Ok(add(a, b)))
            .unwrap();
        globals.set("add_nums", add_nums).unwrap();

        let val = ctx.load("add_nums(12, 3)").eval::<i32>().unwrap();
        println!("{:?}", val);
    });

    // App::new()
    //     .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
    //     .add_plugins(
    //         DefaultPlugins
    //             .set(WindowPlugin {
    //                 primary_window: Some(Window {
    //                     title: "NotCraft".into(),
    //                     present_mode: PresentMode::Immediate,
    //                     resolution: (1280.0, 800.0).into(),
    //                     resizable: false,
    //                     window_theme: Some(WindowTheme::Dark),
    //                     ..default()
    //                 }),
    //                 ..default()
    //             })
    //             .disable::<LogPlugin>(),
    //     )
    //     .add_systems(Startup, notcraft_lib::setup)
    //     .run();
}
