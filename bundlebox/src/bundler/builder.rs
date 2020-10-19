use super::{Bundler, Emitter};
use crate::processor::Processor;
use anyhow::bail;
use log::warn;
use std::fs::create_dir_all;
use std::mem;
use std::path::{Path, PathBuf};

const DEFAULT_SOURCE_DIRECTORY: &str = "source";
const DEFAULT_OUTPUT_DIRECTORY: &str = "build";
const DEFAULT_ENTRYPOINT: &str = "index.ron";

#[derive(Clone)]
pub struct Builder<IO: Processor> {
    source_directory: Option<PathBuf>,
    output_directory: Option<PathBuf>,
    entrypoint: Option<PathBuf>,
    pipeline: Option<IO>,
}

impl<IO: Processor> Default for Builder<IO> {
    fn default() -> Self {
        Builder {
            source_directory: None,
            output_directory: None,
            entrypoint: None,
            pipeline: None,
        }
    }
}

impl<IO: Processor> Builder<IO> {
    pub fn source_directory<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.source_directory = Some(path.as_ref().to_owned());
        self
    }

    pub fn output_directory<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.output_directory = Some(path.as_ref().to_owned());
        self
    }

    pub fn entrypoint<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.entrypoint = Some(path.as_ref().to_owned());
        self
    }

    pub fn pipeline(&mut self, processor: IO) -> &mut Self {
        self.pipeline = Some(processor);
        self
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let source_directory = self.source_directory.clone().unwrap_or_else(|| {
            let source_directory: PathBuf = DEFAULT_SOURCE_DIRECTORY.into();
            warn!(
                "Source directory is not specified, assuming '{}'",
                source_directory.display()
            );
            source_directory
        });

        if !source_directory.exists() {
            bail!(
                "Source directory does not exist: {}",
                source_directory.display()
            );
        }

        if !source_directory.is_dir() {
            bail!(
                "Source path is not a directory: {}",
                source_directory.display()
            );
        }

        let output_directory = self.output_directory.clone().unwrap_or_else(|| {
            let output_directory: PathBuf = DEFAULT_OUTPUT_DIRECTORY.into();
            warn!(
                "Output directory is not specified, assuming '{}'",
                output_directory.display()
            );
            output_directory
        });

        if !output_directory.exists() {
            warn!(
                "Output directory does not exists, creating '{}'",
                output_directory.display()
            );
            create_dir_all(&output_directory)?;
        }

        if !output_directory.is_dir() {
            bail!(
                "Output path is not a directory: {}",
                output_directory.display()
            );
        }

        let entrypoint = self.entrypoint.clone().unwrap_or_else(|| {
            let entrypoint: PathBuf = DEFAULT_ENTRYPOINT.into();
            warn!(
                "Entrypoint is not specified, assuming '{}'",
                entrypoint.display()
            );
            entrypoint
        });

        let entrypoint_path: PathBuf = source_directory.join(&entrypoint);

        if !entrypoint_path.exists() {
            bail!("Entrypoint does not exists: {}", entrypoint_path.display());
        }

        if !entrypoint_path.is_file() {
            bail!("Entrypoint is not a file: {}", entrypoint_path.display());
        }

        let pipeline = match mem::take(&mut self.pipeline) {
            Some(p) => p,
            None => bail!("No pipeline specified"),
        };

        let emitter = Emitter::new(source_directory, output_directory);
        let mut bundler = Bundler::new(entrypoint, emitter, pipeline);

        bundler.run()
    }
}
