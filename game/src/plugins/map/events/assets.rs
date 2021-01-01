use super::super::assets::TiledMap;
use bevy::prelude::*;

#[derive(Default)]
pub struct MapAssetsListener {
    pub reader: EventReader<AssetEvent<TiledMap>>,
}
