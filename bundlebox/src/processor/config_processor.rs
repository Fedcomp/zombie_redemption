use super::{Asset, Processor};
use crate::bundler::Emitter;
use log::error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    dependent_assets: Vec<String>,
}

/// Simply copy files
#[derive(Default)]
pub struct ConfigProcessor;

impl Processor for ConfigProcessor {
    fn process(&mut self, asset: Asset, emitter: &mut Emitter) -> anyhow::Result<()> {
        let asset_path = emitter.source_directory().join(asset.path);
        let config: Config = ron::de::from_reader(asset.contents)?;

        for path in config.dependent_assets.into_iter() {
            let source_path = emitter.source_directory().join(&path);
            let contents = match fs::File::open(&source_path) {
                Ok(c) => c,
                Err(e) => {
                    error!(
                        "{} requires {}, but failed to open: {}",
                        asset_path.display(),
                        source_path.display(),
                        e
                    );
                    continue;
                }
            };

            emitter.emit_asset(Asset::new(path, Box::new(contents)));
        }

        Ok(())
    }
}
