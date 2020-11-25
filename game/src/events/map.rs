use bevy::{utils::HashMap, prelude::*};

#[derive(Debug)]
pub enum MapEvents {
    LoadMap(String),
}

#[derive(Default)]
pub struct MapEventsListener {
    pub reader: EventReader<MapEvents>,
}

#[derive(Default)]
pub struct MapMaterials {
    pub materials: HashMap<u32, Handle<ColorMaterial>>
}