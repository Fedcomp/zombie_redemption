use bevy::prelude::*;
use bevy::ecs::Bundle;
use crate::assets::Prefab;

#[derive(Bundle, Default)]
pub struct PrefabComponents {
    pub prefab_handle: Handle<Prefab>
}

#[derive(Properties,Default)]
pub struct TransmutableComponent {}