use crate::assets::Map;
use bevy::prelude::*;

#[derive(Default)]
pub struct MapAssetsListener {
    pub reader: EventReader<AssetEvent<Map>>,
}
