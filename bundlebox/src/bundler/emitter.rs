use crate::processor::Asset;
use log::info;
use std::fs;
use std::io;
use std::mem;
use std::path::PathBuf;

/// Emit files and assets
pub struct Emitter {
    output_directory: PathBuf,
    new_assets: Vec<Asset>,
}

impl Emitter {
    pub fn new(output_directory: PathBuf) -> Emitter {
        Emitter {
            output_directory,
            new_assets: Vec::new(),
        }
    }

    pub fn emit_file(&self, mut asset: Asset) -> anyhow::Result<()> {
        let output_path = self.output_directory.join(asset.path);
        info!("Producing {}", output_path.display());
        io::copy(&mut asset.contents, &mut fs::File::create(output_path)?)?;
        Ok(())
    }

    pub fn emit_asset(&mut self, asset: Asset) {
        self.new_assets.push(asset);
    }

    pub fn take_emmited_assets(&mut self) -> Vec<Asset> {
        mem::take(&mut self.new_assets)
    }
}
