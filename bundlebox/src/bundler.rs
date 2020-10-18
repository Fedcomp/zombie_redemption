mod builder;
mod emitter;

pub use self::emitter::Emitter;
pub use self::builder::Builder;

use crate::processor::{Asset, Processor};
use log::{info, debug};
use std::fs;
use std::path::PathBuf;

pub struct Bundler<IO: Processor> {
    source_dir: PathBuf,
    entrypoint: PathBuf,
    emitter: Emitter,
    pipeline: IO,
}

impl <IO: Processor + Default> Bundler<IO> {
    pub fn build() -> Builder<IO> {
        Builder::default()
    }

    pub fn new(
        source_dir: PathBuf,
        entrypoint: PathBuf,
        emitter: Emitter,
        pipeline: IO
    ) -> Bundler<IO> {
        Bundler {
            source_dir,
            entrypoint,
            emitter,
            pipeline
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let entrypoint_path = self.source_dir.join(&self.entrypoint);
        info!("Entering entrypoint: {}", self.entrypoint.display());
        let contents = Box::new(fs::File::open(&entrypoint_path)?);
        let asset = Asset::new(&self.entrypoint, contents);
        self.process_asset(asset)
    }

    pub fn process_asset(&mut self, asset: Asset) -> anyhow::Result<()> {
        debug!("Processing asset {}", asset);
        self.pipeline.process(asset, &mut self.emitter)?;
        
        let additional_assets = self.emitter.take_emmited_assets();
        for asset in additional_assets.into_iter() {
            self.process_asset(asset)?;
        }

        Ok(())
    }
}
