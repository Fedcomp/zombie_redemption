use super::{Asset, Processor};
use crate::bundler::Emitter;
use anyhow::format_err;
use log::trace;
use std::process::Command;
use tempfile::Builder as TempFileBuilder;

/// Render files to png using inkscape
#[derive(Default)]
pub struct SvgProcessor;

impl Processor for SvgProcessor {
    fn process(&mut self, asset: Asset, emitter: &mut Emitter) -> anyhow::Result<()> {
        let source_path = emitter.source_directory().join(&asset.path);
        let output_file = TempFileBuilder::new().suffix(".png").tempfile()?;

        let mut command = Command::new("inkscape");
        command
            .arg(source_path)
            .arg("--export-filename")
            .arg(output_file.path());

        trace!("Running {:?}", command);
        let output = command
            .output()
            .map_err(|e| format_err!("Failed to start inkscape: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        trace!(
            "Inkscape run results \nstderr: \"{}\" \nstdout: \"{}\"",
            stderr.trim(),
            stdout.trim()
        );

        let mut new_path = asset.path.clone();
        new_path.set_extension("png");
        let new_asset = Asset::new(new_path, Box::new(output_file));
        emitter.emit_asset(new_asset);

        Ok(())
    }
}
