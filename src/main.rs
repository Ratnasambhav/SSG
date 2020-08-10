use ssg::run;
use std::ffi::OsString;

fn main() {
    let post_dir: OsString = OsString::from("./posts/");
    let template_dir: OsString = OsString::from("./templates/");
    let dist_dir: OsString = OsString::from("./dist/");

    // TODO: Error handling!
    run(&post_dir, &template_dir, &dist_dir);
}
