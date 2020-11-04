use std::io::BufReader;
use std::path::Path;
use bevy::asset::AssetLoader;
use crate::assets::Map;

#[derive(Default)]
pub struct MapLoader;

impl AssetLoader for MapLoader {
    fn from_bytes(&self, asset_path: &Path, bytes: Vec<u8>) -> anyhow::Result<Map> {
        let map = tiled::parse_with_path(BufReader::new(bytes.as_slice()), asset_path)?;
        Ok(Map::new(map))
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}
