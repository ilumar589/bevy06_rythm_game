// create a resource that keeps the materials for the arrow sprites
// this way we don't have to load them every time we want to create an arrow

use bevy::prelude::{Assets, AssetServer, ColorMaterial, FromWorld, Handle, World, Component, Timer, Commands, Res, Time, ResMut, Transform, Vec3, SpriteBundle, Sprite, Query, With, Plugin, Vec2, Quat, Image, Vec4, Entity, KeyCode, Input};
use crate::{App, SongConfig};
use crate::consts::{BASE_SPEED, SPAWN_POSITION, TARGET_POSITION, THREASHOLD};
use crate::types::{Directions, Speed};


pub struct ArrowsPlugin;

impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ArrowMaterialResource>()
            .insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            .add_startup_system(setup_target_arrows)
            .add_system(spawn_arrows)
            .add_system(move_arrows)
            .add_system(despawn_arrows);
    }
}

/// Keeps the textures and materials for Arrows
pub struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}

/// Implementing FromWorld for custom instantiation used in init_resource
impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");

        let mut color_materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        ArrowMaterialResource {
            red_texture: color_materials.add(red_handle.into()),
            blue_texture: color_materials.add(blue_handle.into()),
            green_texture: color_materials.add(green_handle.into()),
            border_texture: color_materials.add(border_handle.into()),
        }
    }
}

#[derive(Component)]
struct Arrow {
    speed: Speed,
    direction: Directions
}

#[derive(Component)]
struct SpawnTimer(Timer);

fn spawn_arrows(
    mut commands: Commands,
    material_handles: Res<ArrowMaterialResource>,
    color_materials: Res<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut song_config: ResMut<SongConfig>
) {
    // We get the current time since startup (secs) and the time since the last iteration (secs_last),
    // this way we check if any arrows should spawn in this window

    // Song starts 3 seconds after start , so we subtract 3 seconds
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // List is ordered , so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to the speed
            let material = match arrow.speed {
                Speed::Slow => color_materials.get(&material_handles.red_texture),
                Speed::Medium => color_materials.get(&material_handles.blue_texture),
                Speed::Fast => color_materials.get(&material_handles.green_texture)
            };

            let mut transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
            // Rotate the arrow according to the direction
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));

            match material {
                None => {
                    panic!("No color material found for arrow {:?}", arrow);
                }
                Some(material_result) => {
                    match material_result.texture.clone() {
                        None => {
                            panic!("No texture found on material_result {:?}", material_result);
                        }
                        Some(texture) => {
                            commands
                                .spawn_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        color: material_result.color,
                                        flip_x: false,
                                        flip_y: false,
                                        custom_size: Some(Vec2::new(140., 140.))
                                    },
                                    transform,
                                    global_transform: Default::default(),
                                    texture,
                                    visibility: Default::default()
                                })
                                .insert(Arrow {
                                    speed: arrow.speed,
                                    direction: arrow.direction
                                });
                        }
                    }
                }
            }
        } else {
            break;
        }
    }

    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();
    }
}

#[derive(Component)]
struct TargetArrow;

fn setup_target_arrows(mut commands: Commands,
                       material_handles: Res<ArrowMaterialResource>,
                       color_materials: Res<Assets<ColorMaterial>>) {

    let directions = [Directions::Up, Directions::Down, Directions::Left, Directions::Right];

    for direction in directions.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));

        let border_material = color_materials.get(&material_handles.border_texture);

        match border_material {
            None => {
                panic!("Border material not found");
            }
            Some(border_material_result) => {
                let texture = border_material_result.texture.clone();

                match texture {
                    None => {
                        panic!("No texture found for border material")
                    }
                    Some(texture_result) => {
                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: border_material_result.color,
                                    flip_x: false,
                                    flip_y: false,
                                    custom_size: Some(Vec2::new(140., 140.))
                                },
                                transform,
                                global_transform: Default::default(),
                                texture: texture_result,
                                visibility: Default::default()
                            })
                            .insert(TargetArrow);
                    }
                }
            }
        }
    }
}

fn despawn_arrows(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>
) {
    for (entity, transform, arrow) in query.iter() {
        let position = transform.translation.x;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THREASHOLD..= TARGET_POSITION + THREASHOLD).contains(&position) &&
            arrow.direction.key_just_pressed(&keyboard_input) {
            commands.entity(entity).despawn();
        }

        // Despawn arrows after they leave the screen
        if position >=  2. * TARGET_POSITION {
            commands.entity(entity).despawn();
        }
    }
}