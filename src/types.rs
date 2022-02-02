use serde_derive::{Serialize, Deserialize};
use std::f32::consts::PI;
use std::fs::File;
use std::io::Read;
use bevy::asset::Handle;
use bevy::prelude::{AssetServer, AudioSource, Input, KeyCode};
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira_cpal::CpalBackend;
use crate::consts::{BASE_SPEED, DISTANCE};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

impl Directions {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::D],
            Directions::Down => [KeyCode::Down, KeyCode::F],
            Directions::Left => [KeyCode::Left, KeyCode::J],
            Directions::Right => [KeyCode::Right, KeyCode::K],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Returns the correct rotation for an arrow with this direction
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.
        }
    }

    /// Returns the correct y coordinate for an arrow with this direction
    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => 150.,
            Directions::Down => 50.,
            Directions::Left => -50.,
            Directions::Right => -150.0
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast
}

impl Speed {
    /// Return the actual speed at which the arrow should move
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }

    /// Speed multiplier
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions
}

impl ArrowTime {
    fn new(arrow_toml: &ArrowTimeToml) -> Self {
        let speed_value = arrow_toml.speed.value();
        Self {
            spawn_time: arrow_toml.click_time - (DISTANCE / speed_value) as f64,
            speed: arrow_toml.speed,
            direction: arrow_toml.direction
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions
}

pub struct SongConfig {
    pub name: String,
    pub manager: AudioManager<CpalBackend>,
    pub song_audio: StaticSoundData,
    pub arrows: Vec<ArrowTime>
}

#[derive(Deserialize, Debug)]
struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>
}

impl SongConfig {
    pub fn load_config(path: &str) -> Self {
        // Open files and read contents
        let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Couldn't read file into String");

        // Parse using toml and serde
        let parsed: SongConfigToml = toml::from_str(&contents).expect("Couldn't parse into SongConfigToml");

        // Process arrows
        let mut arrows = parsed
            .arrows
            .iter()
            .map(|arrow| ArrowTime::new(arrow))
            .collect::<Vec<ArrowTime>>();

        // sort arrow by spawn time
        arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

        // load song audio and get the handle
        let manager = AudioManager::new(
            CpalBackend::new().unwrap(),
            AudioManagerSettings::default()
        ).unwrap();

        let sound_data = kira_loaders::load(
            format!("assets/songs/{}", parsed.filename),
            StaticSoundSettings::default()
        ).unwrap();

        SongConfig {
            name: parsed.name,
            manager,
            song_audio: sound_data,
            arrows
        }
    }
}