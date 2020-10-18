use std::fmt;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::bundler::Bundler;

pub struct Asset {
    path: PathBuf,
    contents: Box<dyn Read>,
}

impl Asset {
    pub fn new<P: AsRef<Path>>(path: P, contents: Box<dyn Read>) -> Asset {
        Asset {
            path: path.as_ref().to_owned(),
            contents,
        }
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Asset \"{}\">", self.path.display())
    }
}

pub trait Processor {
    fn process(&mut self, asset: Asset, additional_assets: &mut Vec<Asset>) -> anyhow::Result<Option<Asset>>;
}
