use textbundle::{self, TextBundle, TextPackWriter};

fn main() {
    let tb = TextBundle::new("text.md", "# A dog", vec![]);
    tb.write("dog.textpack").unwrap();
}
