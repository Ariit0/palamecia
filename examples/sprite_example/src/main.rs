use bevy::prelude::*;
use shared::add_one;

fn main() {
    let result = add_one(1);

    println!("Hello, world! {0}", result);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Palamecia".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, core::setup)
        .add_systems(Update, core::character_movement)
        .run();
}
