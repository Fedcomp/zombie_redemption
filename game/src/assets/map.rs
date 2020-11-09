use bevy::type_registry::TypeUuid;

/// Asset container for tiled map
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "a94ea61a-ba7d-4a1c-abe4-6edb6ce9d878"]
pub struct Map {
    pub source: tiled::Map,
}

impl Map {
    pub fn new(source: tiled::Map) -> Self {
        Map { source }
    }
}
