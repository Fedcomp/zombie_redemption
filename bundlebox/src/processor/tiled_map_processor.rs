use super::{Asset, Processor};
use crate::bundler::Emitter;
use anyhow::format_err;
use log::trace;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use tempfile::tempfile;
use xml::reader::EventReader;
use xml::writer::EmitterConfig;
use xml::writer::XmlEvent as WriterEvent;

/// Simply copy files
#[derive(Default)]
pub struct TiledMapProcessor;

// Assumes pipeline process svg assets into png files
fn process_svg_texture<P1: AsRef<Path>, P2: AsRef<Path>>(
    map_path: P1,
    texture_tiled_path: P2,
    emitter: &mut Emitter,
) -> anyhow::Result<String> {
    let texture_tiled_path = texture_tiled_path.as_ref();
    let new_texture_tiled_path = texture_tiled_path.with_extension("png");
    // TODO: unwrap_or is untested
    let map_root_path = map_path.as_ref().parent().unwrap_or(map_path.as_ref());
    let texture_absolute_path = map_root_path.join(texture_tiled_path);
    let texture_src_absolute_path = emitter.source_directory().join(&texture_absolute_path);

    trace!(
        "Reprocessing {} as png file",
        texture_src_absolute_path.display()
    );
    let texture_contents = File::open(&texture_src_absolute_path).map_err(|err| {
        format_err!(
            "Failed to open map texture file at {}: {}",
            texture_src_absolute_path.display(),
            err
        )
    })?;

    emitter.emit_asset(Asset::new(
        texture_absolute_path,
        Box::new(texture_contents),
    ));

    Ok(new_texture_tiled_path.to_string_lossy().to_string())
}

impl Processor for TiledMapProcessor {
    fn process(&mut self, map: Asset, emitter: &mut Emitter) -> anyhow::Result<()> {
        let mut output_file = tempfile()?;
        let parser = EventReader::new(map.contents);
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut output_file);

        for reader_event in parser {
            let reader_event = reader_event?;
            let writer_event = match reader_event.as_writer_event() {
                Some(v) => v,
                // TODO: Ensure all skipped reader events are safe.
                None => continue,
            };

            let mut attributes_buf: Vec<(String, String)> = Vec::new();
            let processed_event = match writer_event {
                WriterEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                } => {
                    let mut new_start_element = WriterEvent::start_element(name);
                    // TODO: Support namespaces
                    for attribute in attributes.into_iter() {
                        let attribute = attribute.to_owned();
                        let name = attribute.name.to_string();
                        let mut value = attribute.value;

                        // Convert map svg textures to png
                        if name == "source" && value.ends_with(".svg") {
                            value = process_svg_texture(&map.path, &value, emitter)?;
                        }

                        attributes_buf.push((name, value));
                    }

                    for (name, value) in attributes_buf.iter() {
                        new_start_element = new_start_element.attr(name.as_ref(), value);
                    }
                    new_start_element.into()
                }
                other => other,
            };

            writer.write(processed_event)?;
            attributes_buf.clear();
        }

        output_file.seek(SeekFrom::Start(0))?;
        emitter.emit_file(Asset::new(map.path, Box::new(output_file)))?;
        Ok(())
    }
}
