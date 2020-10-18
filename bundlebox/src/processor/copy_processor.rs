use super::{Asset, Processor};
use crate::bundler::Emitter;
use log::trace;

/// Simply copy files
#[derive(Default)]
pub struct CopyProcessor;

impl Processor for CopyProcessor {
    fn process(&mut self, asset: Asset, emitter: &mut Emitter) -> anyhow::Result<()> {
        trace!("Copying {}", asset);
        emitter.emit_file(asset)?;
        Ok(())
    }
}
