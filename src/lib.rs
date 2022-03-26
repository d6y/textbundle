use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use zip::result::ZipResult;
use zip::write::FileOptions;

pub struct TextBundle<'a> {
    text_filename: &'a str,
    text: &'a str,
    assets: Vec<&'a Path>,
}

impl<'a> TextBundle<'a> {
    pub fn new(text: &'a str, assets: Vec<&'a Path>) -> TextBundle<'a> {
        TextBundle {
            text_filename: "text.markdown",
            text,
            assets,
        }
    }
}
fn info() -> serde_json::Value {
    json!({
        "version": 2_u8,
        "transient": true,
        "type" : "net.daringfireball.markdown",
    })
}

pub trait TextBundleWriter {
    fn write_textbundle(&self, filename: &str) -> io::Result<()>;
}

pub trait TextPackWriter {
    fn write_textpack(&self, filename: &str) -> ZipResult<()>;
}

impl<'a> TextPackWriter for TextBundle<'a> {
    fn write_textpack(&self, filename: &str) -> ZipResult<()> {
        let path = Path::new(filename);
        let file = std::fs::File::create(&path)?;

        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        zip.start_file("info.json", options)?;
        zip.write_all(info().to_string().as_bytes())?;

        zip.start_file(self.text_filename, options)?;
        zip.write_all(self.text.as_bytes())?;

        if !self.assets.is_empty() {
            zip.add_directory("assets/", Default::default())?;

            for asset in self.assets.iter() {
                if let Some(name) = asset.file_name() {
                    let asset_filename = Path::new("assets/").join(name);
                    let asset_bytes = std::fs::read(asset)?;
                    zip.start_file(asset_filename.to_string_lossy(), options)?;
                    zip.write_all(&asset_bytes)?;
                }
            }
        }

        zip.finish()?;
        Ok(())
    }
}

impl<'a> TextBundleWriter for TextBundle<'a> {
    fn write_textbundle(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        fs::create_dir_all(path)?;

        fs::write(path.join("info.json"), info().to_string())?;

        if !self.assets.is_empty() {
            let asset_dir = &path.join("assets");
            fs::create_dir(asset_dir)?;

            for asset in self.assets.iter() {
                if let Some(name) = asset.file_name() {
                    let asset_filename = asset_dir.join(name);
                    let asset_bytes = std::fs::read(asset)?;
                    fs::write(asset_filename, &asset_bytes)?;
                }
            }
        }

        Ok(())
    }
}
