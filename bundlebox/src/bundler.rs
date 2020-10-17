use anyhow::bail;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const RESOURCE_EXTENSION: &str = ".resource";

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

        let file_iterator = WalkDir::new(&self.source_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|f| f.file_type().is_file())
            .map(|f| f.file_name().to_owned() )
            .filter(|path| path.to_string_lossy().ends_with(RESOURCE_EXTENSION));

        for path in file_iterator {
            self.process_resource(path)?;
        }

        Ok(())
    }

    pub fn process_resource<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path = self.source_dir.join(path);
        dbg!(path);
        Ok(())
    }
}
