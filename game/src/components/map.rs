use crate::assets::Map;
use bevy::ecs::Bundle;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MapComponents {
    pub map_handle: Handle<Map>
}
