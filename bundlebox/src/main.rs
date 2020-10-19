mod bundler;
mod processor;

use crate::bundler::Bundler;
use crate::processor::{ConfigProcessor, CopyProcessor, EitherProcessor};
use clap::{App, Arg};
use env_logger::Env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("SRC_DIR")
                .help("Directory with source assets")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("DST_DIR")
                .help("Processed assets will be bundled here")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("ENTRY")
                .help("A file that will be used as an entrypoint")
                .required(true)
                .index(3),
        )
        .get_matches();

    let source_directory: PathBuf = args
        .value_of("SRC_DIR")
        .expect("SRC_DIR is required")
        .into();
    let destination_directory: PathBuf = args
        .value_of("DST_DIR")
        .expect("DST_DIR is required")
        .into();
    let entrypoint: PathBuf = args.value_of("ENTRY").expect("ENTRY is required").into();

    let copy_processor = CopyProcessor::default();
    let config_processor = ConfigProcessor::default();
    let ron_filter = EitherProcessor::new(config_processor, copy_processor, |asset| {
        asset.path.to_string_lossy().ends_with(".ron")
    });
    let pipeline = ron_filter;

    Bundler::build()
        .source_directory(source_directory)
        .output_directory(destination_directory)
        .entrypoint(entrypoint)
        .pipeline(pipeline)
        .run()
}
