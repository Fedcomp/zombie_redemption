use super::super::assets::TiledMap;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::utils::BoxedFuture;
use std::io::BufReader;

#[derive(Default)]
pub struct MapLoader;

impl AssetLoader for MapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let map = tiled::parse_with_path(BufReader::new(bytes), load_context.path())?;
            load_context.set_default_asset(LoadedAsset::new(TiledMap::new(map)));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}
