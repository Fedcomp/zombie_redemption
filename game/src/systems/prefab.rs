use std::collections::{HashSet, HashMap};
use bevy::{ecs::HecsQuery, math::vec2, prelude::*};
use bevy_rapier2d::physics::RapierConfiguration;
use tiled::Object;
use crate::{assets::{AddComponent, Map, Prefab}, events::{PrefabEvents, PrefabEventsListener, PrefabAssetsListener}, resources::PrefabSpawner};
use bevy::ecs::{Resources, World, With};

//Entity map
//Entity group
//Entity

pub fn process_prefab_loading(
    mut commands: Commands,
    mut state: ResMut<PrefabEventsListener>,
    pschema_events: Res<Events<PrefabEvents>>,
    asset_server: Res<AssetServer>,
) {
    for pschema_event in state.reader.iter(&pschema_events) {
        match pschema_event {
            PrefabEvents::LoadPrefab(file) => {
                // TODO: Find way to merge all pschemas in single object
                let pschemas_ids = Vec::new();

                let mut pschemas:Vec<Handle<Prefab>> = Vec::new();
                // TODO: load_asset_folder is broken for now :()
                pschemas.push(asset_server.load(format!("assets/prefabs/{}",file)).unwrap());

                for pschema_id in pschemas_ids {
                    
                    let pschema_handle: Handle<Prefab> = Handle::from_id(pschema_id);

                    println!("{:?}",pschema_id);

                    pschemas.push(pschema_handle);

                }

                println!("Schema load called {}",file);
            },
            _ => {}
        };
    }
}

pub fn prefab_spawner_system<T: Component>(world: &mut World, resources: &mut Resources,mut query: Query<(&Entity,T)>) where T: AddComponent + Send + Sync + 'static + HecsQuery {
    let mut prefab_spawner = resources.get_mut::<PrefabSpawner>().unwrap();
    let scene_asset_events = resources.get::<Events<AssetEvent<Prefab>>>().unwrap();

    /*
    let mut updated_spawned_scenes = Vec::new();
    for event in state
        .reader
        .iter(&scene_asset_events)
    {
        if let AssetEvent::Modified { handle } = event {
            if prefab_spawner.spawned_prefabs.contains_key(handle) {
                updated_spawned_scenes.push(*handle);
            }
        }
    }
    */

    prefab_spawner.despawn_queued_groups(world);
    prefab_spawner.spawn_queued_groups(world, resources);
    //prefab_spawner
    //    .update_spawned_scenes(world, resources, &updated_spawned_scenes)
    //    .unwrap();
}