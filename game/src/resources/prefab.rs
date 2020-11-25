use bevy::{asset::HandleId, ecs::{bevy_utils::HashMap}, prelude::{Assets, DynamicScene, Handle, Resources, World}, prelude::{SceneSpawner}, property::{DynamicProperties, Properties}, scene::Entity};
use tiled::{Object, ObjectGroup, PropertyValue};

use crate::{assets::{Prefab}};

#[derive(Clone)]
pub struct InstanceData {
    entity: bevy::ecs::Entity,
    prefab: HandleId,
    group_id: u32,
    object: Object,
}

#[derive(Default,Clone)]
pub struct PrefabSpawner {
    spawned_entities: HashMap<bevy::ecs::Entity,InstanceData>,
    spawned_groups: HashMap<u32,Handle<DynamicScene>>, // Object layer id
    groups_to_spawn: Vec<ObjectGroup>,
    groups_to_despawn: Vec<ObjectGroup>,
}

impl PrefabSpawner {

    pub fn get_prefab(&self,resources: &Resources,class: String) -> Option<HandleId> {
        let pschemas = resources.get::<Assets<Prefab>>().unwrap();
        for (handle,prefab) in pschemas.iter() {
            if prefab.class == class {
                return Some(handle);
            }
        }
        None
    }

    pub fn spawn_group(&mut self, group: &ObjectGroup) {
        self.groups_to_spawn.push(group.clone());
    }
    pub fn despawn_group(&mut self, group: &ObjectGroup) {
        self.groups_to_despawn.push(group.clone());
    }

    pub fn despawn_sync(
        &mut self,
        _world: &mut World,
        resources: &Resources,
        group: ObjectGroup,
    ) {
        let mut scene_spawner = resources.get_mut::<SceneSpawner>().unwrap(); 
        let layer_index = &group.layer_index.unwrap();

        if let Some(handle) = self.spawned_groups.get(layer_index) {
            scene_spawner.despawn(handle.clone());
        }

        self.spawned_groups.remove(layer_index);
    }
    
    pub fn despawn_queued_groups(
        &mut self, 
        world: &mut World,
        resources: &Resources
    ){
        let groups_to_despawn = std::mem::take(&mut self.groups_to_despawn);

        for group in groups_to_despawn {
            self.despawn_sync(world, resources, group);
        }
    }

    pub fn spawn_queued_groups(
        &mut self,
        world: &mut World,
        resources: &Resources
    ) {
        let groups_to_spawn = std::mem::take(&mut self.groups_to_spawn);

        for group in groups_to_spawn {
            self.spawn_dynamic_sync(world, resources, &group)
        }
    }

    pub fn spawn_dynamic_sync(
        &mut self,
        _world: &mut World,
        resources: &Resources,
        group: &ObjectGroup,
    ) {
        let mut scenes = resources.get_mut::<Assets<DynamicScene>>().unwrap();
        let mut scene_spawner = resources.get_mut::<SceneSpawner>().unwrap(); 
        let prefabs = resources.get::<Assets<Prefab>>().unwrap();
        let mut scene = DynamicScene::default();
        let mut entities = Vec::new();
        for object in group.objects.iter() {
            let prefab = prefabs.get(self.get_prefab(&resources, object.obj_type.clone()).unwrap()).unwrap();

            let mut components: Vec<DynamicProperties> = Vec::new();

            for component in prefab.components.iter() {
                components.push( Self::patch_component(&component.to_dynamic(),object));
            }

            entities.push(Entity {
                entity: object.id,
                components
            });
        }

        scene.entities.extend(entities.drain(..));

        let handle = scenes.add(scene);

        self.spawned_groups.insert(group.layer_index.unwrap(), handle.clone_weak());

        scene_spawner.spawn_dynamic(handle);
    }

    fn patch_component(component:&DynamicProperties,instance: &Object) -> DynamicProperties {
        let mut component = (*component).to_dynamic();
        for name in component.prop_names.clone().iter() {
            if let Some(val) = instance.properties.get(&format!("{}.{}",component.type_name,name).to_lowercase()) {
                //TODO: It's bad, imagine if tiled supported much more value types
                match val {
                    PropertyValue::BoolValue(value) => component.set_prop(name, value),
                    PropertyValue::FloatValue(value) => component.set_prop(name, value),
                    PropertyValue::IntValue(value) => component.set_prop(name, value),
                    PropertyValue::ColorValue(value) => component.set_prop(name, value),
                    PropertyValue::StringValue(value) => component.set_prop(name, value),
                }
            }
        }
        component.set("gid",instance.gid);
        component.set("width",instance.width);
        component.set("height",instance.height);
        component.set("x",instance.x);
        component.set("y",instance.y);
        component.set("rotation",instance.rotation);
        component.set("visible",instance.visible);
        component
    }

    fn _spawn(&self, _prefab: Prefab ) {
        unimplemented!("In-game spawning is not implemented yet :(")
    }
}