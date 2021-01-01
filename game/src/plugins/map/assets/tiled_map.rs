use bevy::reflect::TypeUuid;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "a94ea61a-ba7d-4a1c-abe4-6edb6ce9d878"]
pub struct TiledMap {
    pub source: tiled::Map,
}

impl TiledMap {
    pub fn new(source: tiled::Map) -> Self {
        TiledMap { source }
    }
}
