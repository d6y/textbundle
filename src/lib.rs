use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use zip::result::ZipResult;
use zip::write::FileOptions;

pub struct TextBundle<'a, P: AsRef<Path>> {
    text_filename: &'a str,
    text: &'a str,
    assets: Vec<P>,
}

impl<'a, P: AsRef<Path>> TextBundle<'a, P> {
    pub fn new(text: &'a str, assets: Vec<P>) -> TextBundle<'a, P> {
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

impl<'a, P: AsRef<Path>> TextBundle<'a, P> {
    pub fn write_textpack(&self, filename: &str) -> ZipResult<()> {
        let path = Path::new(filename);
        let file = std::fs::File::create(&path)?;

        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        let root_folder = "TextBundle";
        zip.add_directory(root_folder, options)?;

        zip.start_file(format!("{root_folder}/info.json"), options)?;
        zip.write_all(info().to_string().as_bytes())?;

        zip.start_file(format!("{root_folder}/{}", self.text_filename), options)?;
        zip.write_all(self.text.as_bytes())?;

        if !self.assets.is_empty() {
            zip.add_directory(format!("{root_folder}/assets"), Default::default())?;

            for asset in self.assets.iter() {
                if let Some(name) = asset.as_ref().file_name() {
                    let asset_filename = Path::new(root_folder).join("assets").join(name);
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

impl<'a, P: AsRef<Path>> TextBundle<'a, P> {
    pub fn write_textbundle(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        fs::create_dir_all(path)?;

        fs::write(path.join("info.json"), info().to_string())?;
        fs::write(path.join(self.text_filename), self.text)?;

        if !self.assets.is_empty() {
            let asset_dir = &path.join("assets");
            fs::create_dir(asset_dir)?;

            for asset in self.assets.iter() {
                if let Some(name) = asset.as_ref().file_name() {
                    let asset_filename = asset_dir.join(name);
                    let asset_bytes = std::fs::read(asset)?;
                    fs::write(asset_filename, &asset_bytes)?;
                }
            }
        }

        Ok(())
    }
}
