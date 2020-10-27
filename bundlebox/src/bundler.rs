mod builder;
mod emitter;

pub use self::builder::Builder;
pub use self::emitter::Emitter;

use crate::processor::{Asset, Processor};
use log::{debug, error, info};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

pub struct Bundler<IO: Processor> {
    entrypoint: PathBuf,
    emitter: Emitter,
    pipeline: IO,
    asset_queue: VecDeque<Asset>,
}

impl<IO: Processor> Bundler<IO> {
    pub fn build() -> Builder<IO> {
        Builder::default()
    }

    pub fn new(entrypoint: PathBuf, emitter: Emitter, pipeline: IO) -> Bundler<IO> {
        Bundler {
            entrypoint,
            emitter,
            pipeline,
            asset_queue: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let entrypoint_path = self.emitter.source_directory().join(&self.entrypoint);
        info!("Entering entrypoint: {}", self.entrypoint.display());
        let contents = Box::new(fs::File::open(&entrypoint_path)?);
        let asset = Asset::new(&self.entrypoint, contents);
        self.asset_queue.push_back(asset);

        self.process_assets()
    }

    pub fn process_assets(&mut self) -> anyhow::Result<()> {
        while let Some(asset) = self.asset_queue.pop_front() {
            debug!("Processing {}", asset);

            let source_path = self.emitter.source_directory().join(&asset.path);
            if let Err(e) = self.pipeline.process(asset, &mut self.emitter) {
                error!("Failed to process {}: {}", source_path.display(), e);
            }

            let elements = self.emitter.take_emmited_assets();
            self.asset_queue.extend(elements);
        }

        Ok(())
    }
}
