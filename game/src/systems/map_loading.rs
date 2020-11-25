use std::collections::{HashSet};
use bevy::prelude::*;
use bevy_rapier2d::{physics::RapierConfiguration, rapier::{geometry::ColliderBuilder}};
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use tiled::PropertyValue;
use crate::{events::{MapAssetsListener, MapEvents, MapEventsListener, MapMaterials}, resources::PrefabSpawner};
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
    mut prefab_spawner: ResMut<PrefabSpawner>,
    mut state: Local<MapAssetsListener>,
    map_asset_events: Res<Events<AssetEvent<Map>>>,
    maps: ResMut<Assets<Map>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut materials_map: ResMut<MapMaterials>,
    rapier_conf: Res<RapierConfiguration>,
) {
    let mut changed_maps = HashSet::<Handle<Map>>::new();
    for map_asset_event in state.reader.iter(&map_asset_events) {
        match map_asset_event {
            AssetEvent::Created { ref handle } => {
                changed_maps.insert(handle.clone_weak());
            }
            AssetEvent::Modified { ref handle } => {
                changed_maps.insert(handle.clone_weak());
                println!("Modified")
            }
            AssetEvent::Removed { ref handle } => {
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.remove(handle);
            }
        };
    }

    for changed_map in changed_maps.iter() {

        let phys_scale = rapier_conf.scale;

        let map = &maps.get(changed_map)
                            .expect("Failed to get changed map struct");

        let tsize = map.tile_size();

            materials_map.materials.clear();

            // Reload textures from tilesets
            for tileset in map.source.tilesets.iter() {
                for tile in tileset.tiles.iter() {
                    for image in tile.images.iter() {
                        let image_path = format!("maps/{}", image.source);
                        let texture_handle = asset_server.load(image_path.as_str());
                        materials_map.materials.insert(
                            tileset.first_gid + tile.id,
                            materials.add(texture_handle.into()),
                        );
                    }
                }
            }

            for objgroup in map.source.object_groups.iter() {

                prefab_spawner.despawn_group(objgroup);
                prefab_spawner.spawn_group(objgroup);

                /*
                for object in objgroup.objects.iter() {
            
                
                    let vobj = map.obj_project_old(object,phys_scale);
                    let sobj = vec2(object.width,object.height);

                    let material = materials_map.materials.get(&object.gid).expect(&format!("Unknown object material {}", &object.gid));

                    let body = RigidBodyBuilder::new_dynamic()
                        .translation(vobj.x(),vobj.y())
                        .rotation(object.rotation.to_radians());

                    let collider = ColliderBuilder::cuboid(sobj.x(), sobj.y());

                    commands.spawn(SpriteComponents {
                        material: material.clone(),
                        transform: Transform::from_scale((sobj/tsize).extend(0.0)),
                        ..Default::default()
                    }).with(body).with(collider);

                    
                }*/
            }

            // Place blocks in the world
            for layer in map.source.layers.iter() {

                let layer_tiles = match &layer.tiles {
                    tiled::LayerData::Finite(layers) => layers,
                    _ => panic!("No support for infinite maps"),
                };

                let layer_collide = layer.properties.get("collide") == Some(&PropertyValue::BoolValue(true));

                for (line, tiles) in layer_tiles.iter().rev().enumerate() {
                    for (column, tile) in tiles.iter().enumerate() {
                        if tile.gid == 0 {
                            continue;
                        }
                        
                        let tpos = map.tile_project(column as f32 ,line as f32);

                        let material = materials_map.materials.get(&tile.gid).expect(&format!("Unknown tile material {}", &tile.gid));
                        let cmds = commands
                            .spawn(SpriteComponents {
                                material: material.clone(),
                                transform: Transform::from_translation(tpos.extend(0.0)),
                                ..Default::default()
                            });

                        if layer_collide {
                            //TODO: Implement as super collision object later (rapier doesn't support it yet)
                            let rigid_body = RigidBodyBuilder::new_static()
                                            .translation(tpos.x()/phys_scale,tpos.y()/phys_scale);
                            let collider = ColliderBuilder::cuboid(tsize.x(), tsize.y());
                            cmds.with(rigid_body).with(collider);
                        }
                    }
                }
            }
    }
}
