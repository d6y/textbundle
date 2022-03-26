use std::path::Path;
use textbundle::{self, TextBundle, TextBundleWriter, TextPackWriter};

fn main() {
    let dog = Path::new("examples/dog.jpg");

    let markdown = r#"
    # A dog
    
    ![](assets/dog.jpg)
    "#;

    let tb = TextBundle::new(markdown, vec![&dog]);
    tb.write_textpack("dog.textpack").unwrap();
    tb.write_textbundle("dog.textbundle").unwrap();
}
