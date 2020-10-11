use bevy::prelude::*;

#[derive(Debug)]
pub enum MapEvents {
    LoadMap(String)
}

#[derive(Default)]
pub struct MapEventsListener {
    pub reader: EventReader<MapEvents>,
}
