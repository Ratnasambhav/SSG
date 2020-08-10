use ssg::run;
use std::path::Path;
use std::ffi::OsString;

fn main() {
    let post_dir = OsString::from("./posts/");
    let index_html_path = OsString::from("./templates/index.html");
    let post_html_path = OsString::from("./templates/post.html");
    let dist_dir = Path::new("./dist/");

    // TODO: Error handling!
    match run(&post_dir, &index_html_path, &post_html_path, &dist_dir) {
        Ok(()) => println!("Success"),
        Err(e) => eprintln!("ERROR: {}", e)
    }
}
