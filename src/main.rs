use ssg::{Metadata, get_metadata};

fn main() {
    let metadata = get_metadata("posts/test.md").unwrap();

    let meta = Metadata::new(&metadata).unwrap();
    println!("{:?}", meta);
}