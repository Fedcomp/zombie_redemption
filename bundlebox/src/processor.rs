mod config_processor;
mod copy_processor;
mod either_processor;
mod skip_processor;
mod svg_processor;
mod tiled_map_processor;

pub use config_processor::ConfigProcessor;
pub use copy_processor::CopyProcessor;
pub use either_processor::EitherProcessor;
pub use skip_processor::SkipProcessor;
pub use svg_processor::SvgProcessor;
pub use tiled_map_processor::TiledMapProcessor;

use crate::bundler::Emitter;
use std::fmt;
use std::io::Read;
use std::path::{Path, PathBuf};

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
