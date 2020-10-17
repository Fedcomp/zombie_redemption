mod bundler;

use crate::bundler::Bundler;
use clap::{App, Arg};
use env_logger::Env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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
        .get_matches();

    let source_directory: PathBuf = args
        .value_of("SRC_DIR")
        .expect("SRC_DIR is required")
        .into();
    let destination_directory: PathBuf = args
        .value_of("DST_DIR")
        .expect("DST_DIR is required")
        .into();

    Bundler::new(source_directory, destination_directory).run()
}
