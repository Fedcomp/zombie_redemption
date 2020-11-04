use bevy::prelude::*;
use crate::assets::Map;
use crate::events::{MapEvents, MapEventsListener};
use crate::loaders::MapLoader;
use crate::systems::{process_map_loading, process_map_change};

#[derive(Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_asset::<Map>()
            .add_asset_loader::<MapLoader>(MapLoader)
            .add_event::<MapEvents>()
            .init_resource::<MapEventsListener>()
            .add_system(process_map_loading.system())
            .add_system(process_map_change.system());
    }
}
