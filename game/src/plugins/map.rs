use bevy::prelude::*;
use crate::assets::Map;
use crate::loaders::MapLoader;

#[derive(Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Map>()
           .add_asset_loader::<Map, MapLoader>();
    }
}
