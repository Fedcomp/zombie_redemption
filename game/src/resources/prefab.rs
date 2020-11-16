use bevy::{ecs::bevy_utils::HashMap, prelude::{Assets, Component, FromResources, Handle, Resources, World}, property::{DynamicProperties, Properties, PropertyVal}, type_registry::ComponentRegistration, property::Property, type_registry::TypeRegistry};
use log::warn;
use tiled::{Object, ObjectGroup, PropertyValue};

use crate::{components::TransmutableComponent, assets::{AddComponent, Prefab}};

#[derive(Clone)]
pub struct InstanceData {
    entity: bevy::ecs::Entity,
    prefab: Handle<Prefab>,
    group_id: u32,
    object: Object,
}

#[derive(Default,Clone)]
pub struct PrefabSpawner {
    spawned_entities: HashMap<bevy::ecs::Entity,InstanceData>,
    spawned_groups: HashMap<u32,Vec<bevy::ecs::Entity>>, // Object layer id
    groups_to_spawn: Vec<ObjectGroup>,
    groups_to_despawn: Vec<ObjectGroup>,
}

impl PrefabSpawner {

    pub fn get_prefab(&self,resources: &Resources,class: String) -> Option<Handle<Prefab>> {
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
        world: &mut World,
        group: ObjectGroup,
    ) {
        let layer_index = &group.layer_index.unwrap();

        if let Some(entities) = &self.spawned_groups.get(layer_index) {
            for entity in entities.into_iter() {
                let _ = world.despawn(*entity); // Ignore the result, despawn only cares if it exists.
                self.spawned_entities.remove(&entity);
            }
        }

        self.spawned_groups.remove(layer_index);
    }
    
    pub fn despawn_queued_groups(&mut self, world: &mut World){
        let groups_to_despawn = std::mem::take(&mut self.groups_to_despawn);

        for group in groups_to_despawn {
            self.despawn_sync(world, group);
        }
    }

    pub fn spawn_queued_groups(
        &mut self,
        world: &mut World,
        resources: &Resources,
    ) {
        let groups_to_spawn = std::mem::take(&mut self.groups_to_spawn);

        for group in groups_to_spawn {
            self.spawn_dynamic_sync(world, resources, &group)
        }
    }

    pub fn spawn_dynamic_sync(
        &mut self,
        world: &mut World,
        resources: &Resources,
        group: &ObjectGroup,
    ) {
        let mut ents: Vec<bevy::ecs::Entity> = Vec::new();
        for object in group.objects.iter() {
            let entity = world.reserve_entity();
            ents.push(entity);
            let mut instance_data = InstanceData {
                entity,
                prefab: self.get_prefab(&resources, object.obj_type.clone()).unwrap(),
                group_id: group.layer_index.unwrap(),
                object: object.clone()
            };
            Self::spawn_dynamic_internal(world, resources, &mut instance_data);
        }
        self.spawned_groups.insert(group.layer_index.unwrap(), ents);
    }

    fn patch_component(component:&DynamicProperties,instance: &Object) -> DynamicProperties {
        println!("{:?}",instance.properties);
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
        component.set("width",instance.width);
        component.set("height",instance.height);
        component.set("x",instance.x);
        component.set("y",instance.y);
        component.set("rotation",instance.rotation);
        component.set("visible",instance.visible);
        component
    }

    //Component Registration -> Component -> Component -> Component Registration
    /*
    fn unwrap_component<T: Properties + Component + FromResources + IntoComponent>(resources: &Resources, property: &dyn Property) -> DynamicProperties {
        let mut component = T::from_resources(resources);
        component.apply(property);
        component.component_into()
    }
    */

    fn spawn_dynamic_internal(
        world: &mut World,
        resources: &Resources,
        instance_data: &mut InstanceData,
    ) {
        let prefabs = resources.get::<Assets<Prefab>>().unwrap();
        let type_registry = resources.get::<TypeRegistry>().unwrap();
        let component_registry = type_registry.component.read();
        let prefab = prefabs.get(&instance_data.prefab).unwrap();
        let entity = instance_data.entity;

            for component in prefab.components.iter() {

                let component = Self::patch_component(component,&instance_data.object);

                if let Some(component_registration) = component_registry.get_with_name(&component.type_name) {
                
                    if world.has_component_type(entity, component_registration.ty) {
                        if component.type_name != "Camera" {
                            component_registration.apply_component_to_entity(world, entity, &component);
                        }
                    } else {
                        component_registration
                            .add_component_to_entity(world, resources, entity, &component);
                    }

                    world.insert_one(entity, TransmutableComponent::default() ).unwrap();
                } else {warn!("Invalid component {}",&component.type_name)}
            }
    }
    fn _spawn(&self, _prefab: Prefab ) {
        unimplemented!("In-game spawning is not implemented yet :(")
    }
}