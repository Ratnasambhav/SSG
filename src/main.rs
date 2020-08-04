use ssg::{run, Metadata};
use std::ffi::OsString;
use std::fs::{read_dir, DirEntry};
use std::io;

fn main() {
    let post_dir: OsString = OsString::from("./posts/");
    let template_dir: OsString = OsString::from("./templates/");
    let dist_dir: OsString = OsString::from("./dist/");

    run(&post_dir, &template_dir, &dist_dir);
}
