use std::path::PathBuf;
use crate::processor::Asset;
use log::info;
use std::mem;

/// Emit files and assets
pub struct Emitter {
    output_directory: PathBuf,
    new_assets: Vec<Asset>
}

impl Emitter {
    pub fn new(output_directory: PathBuf) -> Emitter {
        Emitter {
            output_directory,
            new_assets: Vec::new()
        }
    }

    pub fn emit_file(&self, asset: Asset) -> anyhow::Result<()> {
        let out_path = self.output_directory.join(asset.path);
        info!("Producing {}", out_path.display());
        Ok(())
    }

    pub fn emit_asset(&mut self, asset: Asset){
        self.new_assets.push(asset);
    }

    pub fn take_emmited_assets(&mut self) -> Vec<Asset> {
        mem::take(&mut self.new_assets)
    }
}
