use bevy::prelude::*;
use crate::assets::Prefab;
use crate::events::{PrefabEvents, PrefabEventsListener};
use crate::loaders::PrefabLoader;
use crate::systems::{process_prefab_loading,prefab_spawner_system};
use crate::resources::PrefabSpawner;
use bevy::scene::*;
use crate::components::*;

#[derive(Default)]
pub struct PrefabPlugin;

pub const PREFAB_STAGE: &str = "prefab";

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .register_component::<TransmutableComponent>()
            .register_component::<Cuboid>()
            .add_resource(PrefabSpawner::default())
            .add_asset::<Prefab>()
            .add_asset_loader::<Prefab, PrefabLoader>()
            .add_event::<PrefabEvents>()
            .init_resource::<PrefabEventsListener>()
            .add_system(process_prefab_loading.system())
            .add_stage_after(stage::EVENT_UPDATE, PREFAB_STAGE)
            .add_system_to_stage(SCENE_STAGE, prefab_spawner_system.thread_local_system());
    }
}
