use bevy::prelude::*;
use crate::assets::Map;

#[derive(Default)]
pub struct MapAssetsListener {
    pub reader: EventReader<AssetEvent<Map>>,
}
