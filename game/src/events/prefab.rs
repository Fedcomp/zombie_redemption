use bevy::prelude::*;
use tiled::{ObjectGroup};

#[derive(Debug)]
pub enum PrefabEvents {
    LoadPrefab(String),
}

#[derive(Default)]
pub struct PrefabEventsListener {
    pub reader: EventReader<PrefabEvents>,
}
