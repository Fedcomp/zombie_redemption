use crate::processor::Asset;
use log::{info, trace};
use std::fs;
use std::fs::create_dir_all;
use std::io;
use std::mem;
use std::path::{Path, PathBuf};

/// Emit files and assets
pub struct Emitter {
    source_directory: PathBuf,
    output_directory: PathBuf,
    new_assets: Vec<Asset>,
}

impl Emitter {
    pub fn new(source_directory: PathBuf, output_directory: PathBuf) -> Emitter {
        Emitter {
            source_directory,
            output_directory,
            new_assets: Vec::new(),
        }
    }

    pub fn source_directory(&self) -> &Path {
        &self.source_directory
    }

    pub fn emit_file(&self, mut asset: Asset) -> anyhow::Result<()> {
        let output_path = self.output_directory.join(asset.path);

        if let Some(out_file_dir) = output_path.parent() {
            if !out_file_dir.exists() {
                trace!("{} does not exists, creating one", out_file_dir.display());
                create_dir_all(out_file_dir)?;
            }
        }

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
