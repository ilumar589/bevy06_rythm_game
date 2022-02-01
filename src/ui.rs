use bevy::prelude::{ AssetServer, BuildChildren, Color, Commands, HorizontalAlign, NodeBundle, PositionType, Rect, Res, ResMut, Style, Text, TextAlignment, TextBundle, TextStyle, UiColor, Val, VerticalAlign, Component, Time, Query, With, Plugin, Changed};
use crate::{App, ScoreResource};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_system(update_time_text)
            .add_system(update_score_text);
    }
}


#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct ScoreText;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        // Time text node
        .spawn_bundle(NodeBundle {
            node: Default::default(),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            image: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default()
        })
        .with_children(|parent| {
            let text = Text::with_section(
                     "Time: 0.0".to_string(),
                     TextStyle {
                         font: font.clone(),
                         font_size: 40.0,
                         color: Color::rgb(0.9, 0.9, 0.9),
                     },
                     Default::default()
            );

            parent
                .spawn_bundle(TextBundle {
                    text,
                    ..Default::default()
                })
                .insert(TimeText);
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left:  Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            let text = Text::with_section(
                "Score: 0. Corrects: 0. Fails: 0".to_string(),
                TextStyle {
                    font: font.clone(),
                    color: Color::rgb(0.8, 0.8, 0.8),
                    font_size: 40.0,
                    ..Default::default()
                },
                Default::default()
            );

            parent
                .spawn_bundle(TextBundle {
                    text,
                    ..Default::default()
                })
                .insert(ScoreText);

        });
}

fn update_time_text(time: Res<Time>, mut query: Query<&mut Text, With<TimeText>>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for mut text in query.iter_mut() {
        if let Some(text_value) = text.sections.get_mut(0) {
            text_value.value = format!("Time: {:.2}", secs);
        } else {
            panic!("Section not found!");
        }
    }
}

fn update_score_text(score: Res<ScoreResource>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        if let Some(text_value) = text.sections.get_mut(0) {
            text_value.value = format!(
                "Score: {}. Corrects: {}. Fails: {}",
                score.score(),
                score.corrects(),
                score.fails()
            );
        } else {
            panic!("Section not found!")
        }
    }
}