pub struct Map {
    pub source: tiled::Map
}

impl Map {
    pub fn new(source: tiled::Map) -> Self {
        Map { source }
    }
}
