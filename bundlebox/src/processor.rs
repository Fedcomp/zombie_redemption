mod skip_processor;

pub use skip_processor::SkipProcessor;

use std::fmt;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::bundler::Emitter;

pub struct Asset {
    pub path: PathBuf,
    pub contents: Box<dyn Read>,
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
    fn process(&mut self, asset: Asset, emitter: &mut Emitter) -> anyhow::Result<()>;
}
