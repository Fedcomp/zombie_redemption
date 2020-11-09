use crate::assets::Map;
use crate::components::MapComponents;
use crate::events::{MapAssetsListener, MapEvents, MapEventsListener};
use bevy::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::{physics::RapierConfiguration, rapier::geometry::ColliderBuilder};
use std::collections::{HashMap, HashSet};
use tiled::PropertyValue;

pub fn process_map_loading(
    mut commands: Commands,
    mut state: ResMut<MapEventsListener>,
    map_events: Res<Events<MapEvents>>,
    asset_server: Res<AssetServer>,
) {
    for map_event in state.reader.iter(&map_events) {
        match map_event {
            MapEvents::LoadMap(map_name) => {
                let map_handle = asset_server.load(format!("maps/{}.tmx", map_name).as_str());
                commands.spawn(MapComponents {
                    map_handle,
                    ..Default::default()
                });
            }
        };
    }
}

// TODO: Rework load system
pub fn process_map_change(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // TODO: Local vs Res?
    mut state: Local<MapAssetsListener>,
    map_asset_events: Res<Events<AssetEvent<Map>>>,
    maps: Res<Assets<Map>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rapier_conf: Res<RapierConfiguration>,
    mut query: Query<(
        Entity,
        &Handle<Map>,
        &mut HashMap<u32, Handle<ColorMaterial>>,
        // &Transform,
    )>,
) {
    let mut changed_maps = HashSet::<Handle<Map>>::new();
    for map_asset_event in state.reader.iter(&map_asset_events) {
        match map_asset_event {
            AssetEvent::Created { ref handle } => {
                changed_maps.insert(handle.clone_weak());
            }
            AssetEvent::Modified { ref handle } => {
                changed_maps.insert(handle.clone_weak());
            }
            AssetEvent::Removed { ref handle } => {
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.remove(handle);
            }
        };
    }

    let phys_scale = rapier_conf.scale;

    for changed_map in changed_maps.iter() {
        let map = &maps
            .get(changed_map)
            .expect("Failed to get changed map struct")
            .source;

        for (_, _, mut materials_map) in query.iter_mut() {
            materials_map.clear();

            // Reload textures from tilesets
            for tileset in map.tilesets.iter() {
                for tile in tileset.tiles.iter() {
                    for image in tile.images.iter() {
                        let image_path = format!("maps/{}", image.source);
                        let texture_handle = asset_server.load(image_path.as_str());
                        materials_map.insert(
                            tileset.first_gid + tile.id,
                            materials.add(texture_handle.into()),
                        );
                    }
                }
            }

            for objgroup in map.object_groups.iter() {
                for object in objgroup.objects.iter() {
                    let object_x = object.x;
                    //TODO: MAKE THIS LESS UGLY
                    let object_y = ((map.height * map.tile_height) as f32) - object.y;

                    let width = object.width;
                    let height = object.height;

                    let material = materials_map
                        .get(&object.gid)
                        .expect(&format!("Unknown object material {}", &object.gid));

                    let rot = object.rotation.to_radians();

                    let rot_offset = (
                        rot.cos() * width + rot.sin() * height - width,
                        rot.cos() * height - rot.sin() * width - height,
                    );

                    let body = RigidBodyBuilder::new_dynamic()
                        .translation(
                            object_x / phys_scale + rot_offset.0,
                            object_y / phys_scale + rot_offset.1,
                        )
                        .rotation(rot);
                    let collider = ColliderBuilder::cuboid(width, height);

                    commands
                        .spawn(SpriteComponents {
                            material: material.clone(),
                            transform: Transform::from_scale(Vec3::new(
                                width / (map.tile_width as f32),
                                height / (map.tile_height as f32),
                                0.0,
                            )),
                            ..Default::default()
                        })
                        .with(body)
                        .with(collider);
                }
            }

            // Place blocks in the world
            for layer in map.layers.iter() {
                let layer_collide =
                    layer.properties.get("collide") == Some(&PropertyValue::BoolValue(true));

                let layer_tiles = match &layer.tiles {
                    tiled::LayerData::Finite(layers) => layers,
                    _ => panic!("No support for infinite maps"),
                };

                for (line, tiles) in layer_tiles.iter().rev().enumerate() {
                    for (column, tile) in tiles.iter().enumerate() {
                        if tile.gid == 0 {
                            continue;
                        }

                        let tile_x = ((column as u32) * map.tile_width) as f32;
                        let tile_y = ((line as u32) * map.tile_height) as f32;

                        let material = materials_map
                            .get(&tile.gid)
                            .expect(&format!("Unknown tile material {}", &tile.gid));
                        let cmds = commands.spawn(SpriteComponents {
                            material: material.clone(),
                            transform: Transform::from_translation(Vec3::new(tile_x, tile_y, 0.0)),
                            ..Default::default()
                        });

                        if layer_collide {
                            //TODO: Implement as super collision object later (rapier doesn't support it yet)
                            let rigid_body = RigidBodyBuilder::new_static()
                                .translation(tile_x / phys_scale, tile_y / phys_scale);
                            let collider = ColliderBuilder::cuboid(
                                map.tile_width as f32,
                                map.tile_height as f32,
                            );
                            cmds.with(rigid_body).with(collider);
                        }
                    }
                }
            }
        }
    }
}
