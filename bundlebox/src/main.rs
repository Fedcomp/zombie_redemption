use anyhow::bail;
use clap::{App, Arg};
use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
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

    bundle(source_directory, destination_directory)
}

fn bundle(source_dir: PathBuf, destination_dir: PathBuf) -> anyhow::Result<()> {
    if !source_dir.exists() {
        bail!("Source directory does not exist: {}", source_dir.display());
    }

    if !source_dir.is_dir() {
        bail!("Source path is not a directory: {}", source_dir.display());
    }

    if !destination_dir.exists() {
        create_dir_all(&destination_dir)?;
    }

    if !destination_dir.is_dir() {
        bail!(
            "Destination path is not a directory: {}",
            destination_dir.display()
        );
    }

    Ok(())
}
