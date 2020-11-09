use crate::assets::Map;
use bevy::ecs::Bundle;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Bundle, Default)]
pub struct MapComponents {
    pub map_handle: Handle<Map>,
    pub materials: HashMap<u32, Handle<ColorMaterial>>,
}
