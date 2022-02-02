use bevy::prelude::{Audio, Plugin, Res, ResMut, Time};
use crate::{App, SongConfig};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_song);
    }
}

fn start_song(time: Res<Time>, mut config: ResMut<SongConfig>) {
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= 3. && 3. <= secs {
        let mut song = config.song_audio.clone();
        config.manager.play(song);
    }
}