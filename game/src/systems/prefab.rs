use bevy::{ecs::{DynamicBundle}, prelude::*, type_registry::IntoComponent};
use crate::{components::{PrefabComponents}, events::{PrefabEvents, PrefabEventsListener}, resources::PrefabSpawner};
use bevy::ecs::{Resources, World};

//Entity map
//Entity group
//Entity

pub fn process_prefab_loading(
    mut commands: Commands,
    mut state: ResMut<PrefabEventsListener>,
    pschema_events: Res<Events<PrefabEvents>>,
    asset_server: Res<AssetServer>
) {
    for pschema_event in state.reader.iter(&pschema_events) {
        match pschema_event {
            PrefabEvents::LoadPrefab(file) => {
                let prefab_handle = asset_server.load(format!("prefabs/{}.pfb",file).as_str());
                commands.spawn(PrefabComponents {
                    prefab_handle,
                    ..Default::default()
                });
                println!("Schema load called {}",file);
            }
        };
    }
}

pub fn component_into<T,C: Component>(world: &mut World, resources: &mut Resources) where T:IntoComponent<C> + Component {
    let mut ops: Vec<(Entity,C)> = Vec::new();
    
    world.query::<(Entity, &T)>()
        .for_each(|(entity,component)| {ops.push((entity, component.into_component(resources)))});

    for (entity, component) in ops.into_iter() {
        world.insert(entity, (component,)).unwrap();
        world.remove_one::<T>(entity).unwrap();
    }
}

pub fn bundle_into<T,C: Component + DynamicBundle>(world: &mut World, resources: &mut Resources) where T:IntoComponent<C> + Component + DynamicBundle {
    let mut ops: Vec<(Entity,C)> = Vec::new();
    
    world.query::<(Entity, &T)>()
        .for_each(|(entity,component)| {ops.push((entity, component.into_component(resources)))});

    for (entity, component) in ops.into_iter() {
        world.insert(entity, component).unwrap();
        world.remove_one::<T>(entity).unwrap();
    }
}

pub fn print_system(query: Query<(Entity, &SpriteComponents)>) {
    for (entity, component_a) in query.iter() {
        println!("  Entity({})", entity.id());
        println!(
            "    ComponentA: {{ {:?} }}\n",
            component_a.transform
        );
    }
}

pub fn prefab_spawner_system(world: &mut World, resources: &mut Resources)  {
    let mut prefab_spawner = resources.get_mut::<PrefabSpawner>().unwrap();

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

    prefab_spawner.despawn_queued_groups(world, resources);
    prefab_spawner.spawn_queued_groups(world, resources);


    //let mut query = world.query::<(&Entity,&TransmutableComponent)>();

    //for (entity,_) in &mut query.iter() {
        //let kek = world.get::<AddComponent>(*entity).unwrap();
    //}
    //prefab_spawner
    //    .update_spawned_scenes(world, resources, &updated_spawned_scenes)
    //    .unwrap();
}