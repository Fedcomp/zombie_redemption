use super::{Asset, Processor};
use crate::bundler::Emitter;
use log::trace;

/// Simply skip any incoming assets
#[derive(Default)]
pub struct SkipProcessor {}

impl Processor for SkipProcessor {
    fn process(&mut self, asset: Asset, _emitter: &mut Emitter) -> anyhow::Result<()> {
        trace!("Skipping {}", asset);
        Ok(())
    }
}
