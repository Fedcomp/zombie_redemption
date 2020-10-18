use crate::processor::{Asset, Processor};
use anyhow::bail;
use log::{debug, info};
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

pub struct Bundler {
    source_dir: PathBuf,
    destination_dir: PathBuf,
    entrypoint: PathBuf,
    pipeline: Vec<Box<dyn Processor>>,
}

impl Bundler {
    pub fn new<P: AsRef<Path>>(source_dir: P, destination_dir: P, entrypoint: P) -> Self {
        let source_dir = source_dir.as_ref().to_owned();
        let destination_dir = destination_dir.as_ref().to_owned();
        let entrypoint = entrypoint.as_ref().to_owned();

        Self {
            source_dir,
            destination_dir,
            entrypoint,
            pipeline: Vec::new(),
        }
    }

    pub fn add_processor(mut self, processor: Box<dyn Processor>) -> Self {
        self.pipeline.push(processor);
        self
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        if !self.source_dir.exists() {
            bail!(
                "Source directory does not exist: {}",
                self.source_dir.display()
            );
        }

        if !self.source_dir.is_dir() {
            bail!(
                "Source path is not a directory: {}",
                self.source_dir.display()
            );
        }

        if !self.destination_dir.exists() {
            create_dir_all(&self.destination_dir)?;
        }

        if !self.destination_dir.is_dir() {
            bail!(
                "Destination path is not a directory: {}",
                self.destination_dir.display()
            );
        }

        let entrypoint_path = self.source_dir.join(&self.entrypoint);

        if !entrypoint_path.exists() {
            bail!("Entrypoint does not exists: {}", self.entrypoint.display());
        }

        if !entrypoint_path.is_file() {
            bail!("Entrypoint is not a file: {}", self.entrypoint.display());
        }

        info!("Entering entrypoint: {}", self.entrypoint.display());
        let contents = Box::new(fs::File::open(&entrypoint_path)?);
        let asset = Asset::new(&entrypoint_path, contents);
        self.process_asset(asset)
    }

    pub fn process_asset(&mut self, asset: Asset) -> anyhow::Result<()> {
        debug!("Processing asset {}", asset);

        let mut asset = Some(asset);
        let mut additional_assets: Vec<Asset> = Vec::new();
        for processor in self.pipeline.iter_mut() {
            match asset {
                Some(a) => asset = processor.process(a, &mut additional_assets)?,
                None => break,
            };
        }

        for asset in additional_assets.into_iter() {
            self.process_asset(asset)?;
        }

        Ok(())
    }
}
