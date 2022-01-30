use std::fmt::format;
use bevy::prelude::{Assets, AssetServer, BuildChildren, Color, ColorMaterial, Commands, HorizontalAlign, NodeBundle, PositionType, Rect, Res, ResMut, Style, Text, TextAlignment, TextBundle, TextStyle, UiColor, Val, VerticalAlign, Component, Time, Query, With, Plugin};
use crate::App;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_system(update_time_text);
    }
}


#[derive(Component)]
struct TimeText;

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