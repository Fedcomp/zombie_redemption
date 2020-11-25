use bevy::prelude::*;
use bevy_rapier2d::rapier::{geometry::ColliderBuilder, dynamics::RigidBodyBuilder};
use crate::{assets::Prefab, systems::{bundle_into, component_into, print_system}};
use crate::events::{PrefabEvents, PrefabEventsListener};
use crate::loaders::PrefabLoader;
use crate::systems::{process_prefab_loading,prefab_spawner_system};
use crate::resources::PrefabSpawner;
use crate::components::*;

#[derive(Default)]
pub struct PrefabPlugin;

pub const PREFAB_STAGE: &str = "prefab";

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .register_component::<RigidBody>() // Will be used later
            .add_system(component_into::<RigidBody,RigidBodyBuilder>.thread_local_system())
            .register_component::<Cuboid>() // Will be used later
            .add_system(component_into::<Cuboid,ColliderBuilder>.thread_local_system())
            .register_component::<SpriteC>() // Will be used later
            .add_system(bundle_into::<SpriteC,SpriteComponents>.thread_local_system())
            .add_resource(PrefabSpawner::default())
            .add_asset::<Prefab>()
            .init_asset_loader::<PrefabLoader>()
            .add_event::<PrefabEvents>()
            .init_resource::<PrefabEventsListener>()
            .add_system(process_prefab_loading.system())
            .add_stage_after(stage::EVENT, PREFAB_STAGE)
            .add_system_to_stage(PREFAB_STAGE, prefab_spawner_system.thread_local_system())
            .add_system(print_system.system());
    }
}
