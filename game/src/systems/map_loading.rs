use std::collections::{HashSet, HashMap};
use bevy::prelude::*;
use rapier2d::{geometry::ColliderBuilder, dynamics::RigidBodyBuilder};
use tiled::PropertyValue;
use crate::events::{MapEvents, MapEventsListener, MapAssetsListener};
use crate::assets::Map;
use crate::components::MapComponents;

pub fn process_map_loading(
    mut commands: Commands,
    mut state: ResMut<MapEventsListener>,
    map_events: Res<Events<MapEvents>>,
    asset_server: Res<AssetServer>,
) {
    for map_event in state.reader.iter(&map_events) {
        match map_event {
            MapEvents::LoadMap(map_name) => {
                let map_handle = asset_server.load::<Map, String>(format!("assets/maps/{}.tmx", map_name))
                                                                .expect(&format!("Failed to load map {}", map_name));
                commands
                    .spawn(MapComponents { map_handle, ..Default::default() });
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
            AssetEvent::Created { handle } => {
                changed_maps.insert(*handle);
            }
            AssetEvent::Modified { handle } => {
                changed_maps.insert(*handle);
            }
            AssetEvent::Removed { handle } => {
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.remove(handle);
            }
        };
    }

    for changed_map in changed_maps.iter() {
        let map = &maps.get(changed_map)
                            .expect("Failed to get changed map struct").source;

        for (_, _, mut materials_map) in &mut query.iter() {
            materials_map.clear();

            // Reload textures from tilesets
            for tileset in map.tilesets.iter() {
                for tile in tileset.tiles.iter() {
                    for image in tile.images.iter() {
                        let image_path = format!("assets/maps/{}", image.source);
                        let texture_handle = asset_server.load(&image_path)
                                                .expect(&format!("Failed to load tileset image at path: {}", image.source));

                        materials_map.insert(tileset.first_gid + tile.id, materials.add(texture_handle.into()));
                    }
                }
            }

            let (map_size_x,map_size_y)  = (map.width,map.height);

            // Place blocks in the world
            for layer in map.layers.iter() {
                let layer_tiles = match &layer.tiles {
                    tiled::LayerData::Finite(layers) => layers,
                    _ => panic!("No support for infinite maps")
                };

                for (line, tiles) in layer_tiles.iter().rev().enumerate() {
                    for (column, tile) in tiles.iter().enumerate() {
                        if tile.gid == 0 {
                            continue;
                        }

                        //Center the map at zero cords
                        let tile_x = (column as f32) * 32.0 - (map_size_x as f32) * 16.0;
                        let tile_y = (line as f32) * 32.0 - (map_size_y as f32) * 16.0;

                        if layer.properties.get("collide").contains(&&PropertyValue::BoolValue(true)) {
                            //Implement as super collision object later (rapier doesn't expose it yet)
                            let rigid_body = RigidBodyBuilder::new_static()
                                            .translation(tile_x,tile_y);
                            let collider = ColliderBuilder::cuboid(16.0, 16.0);
                            commands.spawn((rigid_body, collider));
                        }

                        let material = materials_map.get(&tile.gid).expect(&format!("Unknown tile material {}", &tile.gid));
                        commands
                            .spawn(SpriteComponents {
                                material: *material,
                                transform: Transform::from_scale(1.0).with_translation(Vec3::new(tile_x, tile_y, 0.0)),
                                ..Default::default()
                            });
                    }
                }
            }
        }
    }
}
