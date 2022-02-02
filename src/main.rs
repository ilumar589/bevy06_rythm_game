extern crate core;

mod arrows;
mod consts;
mod types;
mod ui;
mod score;
mod audio;

use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::{App, Commands, Msaa, OrthographicCameraBundle, Res, Timer, UiCameraBundle, WindowDescriptor};
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::manager::error::PlaySoundError;
use kira::sound::static_sound::{StaticSoundHandle, StaticSoundSettings};
use kira_cpal::CpalBackend;
use crate::arrows::{ ArrowsPlugin };
use crate::audio::AudioPlugin;
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
        .init_resource::<ScoreResource>()
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let config = SongConfig::load_config("test.toml");
    commands.insert_resource(config);
}
