pub struct Map {
    map: tiled::Map
}

impl Map {
    pub fn new(map: tiled::Map) -> Self {
        Map { map }
    }
}
