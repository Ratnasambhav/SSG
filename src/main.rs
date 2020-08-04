use ssg::parse_metadata;

fn main() {
    let metadata = parse_metadata("posts/test.md");
    print!("{}", metadata.unwrap());
}