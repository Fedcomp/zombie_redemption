use bevy::prelude::*;
use crate::assets::{Map,Prefab};

#[derive(Default)]
pub struct MapAssetsListener {
    pub reader: EventReader<AssetEvent<Map>>,
}

#[derive(Default)]
pub struct PrefabAssetsListener {
    pub reader: EventReader<AssetEvent<Prefab>>,
}
