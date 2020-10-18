use super::{Asset, Processor};
use crate::bundler::Emitter;

/// Process asset by either of processors
/// based on closure condition
pub struct EitherProcessor<L: Processor, R: Processor, F: Fn(&Asset) -> bool> {
    left: L,
    right: R,
    condition: F,
}

impl<L: Processor, R: Processor, F: Fn(&Asset) -> bool> Processor for EitherProcessor<L, R, F> {
    fn process(&mut self, asset: Asset, emitter: &mut Emitter) -> anyhow::Result<()> {
        match (self.condition)(&asset) {
            true => self.left.process(asset, emitter),
            false => self.right.process(asset, emitter),
        }
    }
}

impl<L: Processor, R: Processor, F: Fn(&Asset) -> bool> EitherProcessor<L, R, F> {
    pub fn new(left: L, right: R, condition: F) -> EitherProcessor<L, R, F> {
        EitherProcessor {
            left,
            right,
            condition: condition,
        }
    }
}
