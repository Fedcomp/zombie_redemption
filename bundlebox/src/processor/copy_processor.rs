use super::{Asset, Processor};
use crate::bundler::Emitter;

/// Simply copy files
#[derive(Default)]
pub struct CopyProcessor;

impl Processor for CopyProcessor {
    fn process(&mut self, asset: Asset, emitter: &mut Emitter) -> anyhow::Result<()> {
        emitter.emit_file(asset)?;
        Ok(())
    }
}
