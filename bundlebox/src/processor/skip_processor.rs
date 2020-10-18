use super::{Asset, Processor};
use crate::bundler::Emitter;

/// Simply skip any incoming assets
#[derive(Default)]
pub struct SkipProcessor {}

impl Processor for SkipProcessor {
    fn process(&mut self, _asset: Asset, _emitter: &mut Emitter) -> anyhow::Result<()> {
        Ok(())
    }
}
