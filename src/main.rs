extern crate core;

mod arrows;
mod consts;
mod types;
mod ui;
mod score;

use bevy::DefaultPlugins;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::{App, Commands, Msaa, OrthographicCameraBundle, Timer, UiCameraBundle, WindowDescriptor};
use crate::arrows::{ ArrowsPlugin };
use crate::score::ScoreResource;
use crate::types::SongConfig;
use crate::ui::UIPlugin;

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
        .init_resource::<ScoreResource>()
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
