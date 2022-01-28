extern crate core;

mod arrows;
mod consts;
mod types;

use bevy::DefaultPlugins;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::{App, Commands, Msaa, OrthographicCameraBundle, Timer, WindowDescriptor};
use crate::arrows::{ ArrowsPlugin };
use crate::types::SongConfig;

fn main() {
    App::new()
        .insert_resource( Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Rythm".to_string(),
            ..Default::default()
        })
        .insert_resource(SongConfig::load_config())
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
