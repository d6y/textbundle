use std::path::Path;
use textbundle::{self, TextBundle, TextPackWriter};

fn main() {
    let dog = Path::new("examples/dog.jpg");

    let markdown = r#"
    # A dog
    
    ![](assets/dog.jpg)
    "#;

    let tb = TextBundle::new("text.md", markdown, vec![&dog]);
    tb.write("dog.textpack").unwrap();
}
