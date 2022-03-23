use serde_json::json;
use std::io::Write;
use std::path::Path;
use zip;
use zip::result::ZipResult;
use zip::write::FileOptions;

pub struct TextBundle<'a> {
    text_filename: &'a str,
    text: &'a str,
    assets: Vec<&'a Path>,
}

impl<'a> TextBundle<'a> {
    pub fn new(text_filename: &'a str, text: &'a str, assets: Vec<&'a Path>) -> TextBundle<'a> {
        TextBundle {
            text_filename,
            text,
            assets,
        }
    }
}

pub trait TextPackWriter {
    fn write(&self, filename: &str) -> ZipResult<()>;
}

impl<'a> TextPackWriter for TextBundle<'a> {
    fn write(&self, filename: &str) -> ZipResult<()> {
        let path = Path::new(filename);
        let file = std::fs::File::create(&path)?;

        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        let info = json!({
            "version": 2_u8,
        });
        zip.start_file("info.json", options)?;
        zip.write_all(info.to_string().as_bytes())?;

        if self.assets.is_empty() == false {
            zip.add_directory("assets/", Default::default())?;
        }

        // zip.start_file("test/☃.txt", options)?;
        // zip.write_all(b"Hello, World!\n")?;

        // zip.start_file("test/lorem_ipsum.txt", Default::default())?;
        // zip.write_all(LOREM_IPSUM)?;

        zip.finish()?;
        Ok(())
    }
}
