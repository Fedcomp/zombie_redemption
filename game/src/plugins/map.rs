mod assets;
mod bundles;
mod events;
mod loaders;
mod systems;

pub use self::events::MapEvents;

use self::assets::TiledMap;
use self::events::MapEventsListener;
use self::loaders::MapLoader;
use self::systems::{process_map_change, process_map_loading};
use bevy::prelude::*;

#[derive(Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<TiledMap>()
            .add_asset_loader::<MapLoader>(MapLoader)
            .add_event::<MapEvents>()
            .init_resource::<MapEventsListener>()
            .add_system(process_map_loading.system())
            .add_system(process_map_change.system());
    }
}
