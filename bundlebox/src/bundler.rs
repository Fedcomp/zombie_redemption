use anyhow::bail;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub struct Bundler {
    source_dir: PathBuf,
    destination_dir: PathBuf,
}

impl Bundler {
    pub fn new<P: AsRef<Path>>(source_dir: P, destination_dir: P) -> Self {
        Self {
            source_dir: source_dir.as_ref().to_owned(),
            destination_dir: destination_dir.as_ref().to_owned(),
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
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

        Ok(())
    }
}
