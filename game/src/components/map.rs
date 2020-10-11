use std::collections::HashMap;
use bevy::prelude::*;
use bevy::ecs::Bundle;
use crate::assets::Map;

#[derive(Bundle, Default)]
pub struct MapComponents {
    pub map_handle: Handle<Map>,
    pub materials: HashMap<u32, Handle<ColorMaterial>>,
}
