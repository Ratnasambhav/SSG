use std::ffi::{OsStr, OsString};
use std::fs::{read_dir, DirEntry, File};
use std::io::{self, BufRead, BufReader, Error};
use std::path::PathBuf;

use toml::Value;

#[derive(Debug)]
pub struct Metadata {
    pub title: String,
    pub published: String,
    pub last_update: String,

    // HTML meta tags
    pub description: String,
    pub keywords: String,

    // Facebook meta tags
    pub og_url: String,
    pub og_type: String,
    pub og_title: String,
    pub og_image: String,
    pub og_description: String,

    // Twitter meta tags
    pub twitter_card: String,
    pub twitter_image: String,
    pub twitter_title: String,
    pub twitter_description: String,
    // TODO: Converst meta tags to enum
}

impl Metadata {
    pub fn new(metadata_string: &str) -> Result<Metadata, toml::de::Error> {
        let toml: Value = toml::from_str(metadata_string)?;
        let toml_to_string = |value: &toml::value::Value| String::from(value.as_str().unwrap());

        Ok(Metadata {
            title: toml_to_string(&toml["title"]),
            published: toml_to_string(&toml["published"]),
            last_update: toml_to_string(&toml["last_update"]),
            description: toml_to_string(&toml["meta"]["description"]),
            keywords: toml_to_string(&toml["meta"]["keywords"]),
            og_url: toml_to_string(&toml["meta"]["og_url"]),
            og_type: toml_to_string(&toml["meta"]["og_type"]),
            og_title: toml_to_string(&toml["meta"]["og_title"]),
            og_image: toml_to_string(&toml["meta"]["og_image"]),
            og_description: toml_to_string(&toml["meta"]["og_description"]),
            twitter_card: toml_to_string(&toml["meta"]["twitter_card"]),
            twitter_image: toml_to_string(&toml["meta"]["twitter_image"]),
            twitter_title: toml_to_string(&toml["meta"]["twitter_title"]),
            twitter_description: toml_to_string(&toml["meta"]["twitter_description"]),
        })
    }
}

// Separate a markdofwn file into metadata and markdown strings
fn parse_markdown(buffered: BufReader<File>) -> Result<(String, String), Error> {
    let mut counter = 0;
    let mut metadata = String::new();
    let mut md = String::new();

    for line in buffered.lines() {
        match line?.as_str() {
            "+++" => {
                if counter < 2 {
                    counter += 1;
                } else {
                    md = format!("{}\n+++", md);
                }
            }
            s => {
                if counter < 2 {
                    metadata = format!("{}\n{}", metadata, s);
                } else {
                    md = format!("{}\n{}", md, s);
                }
            }
        };
    }

    Ok((metadata, md))
}

// Runs the show
pub fn run(post_dir: &OsString, template_dir: &OsString, dist_dir: &OsString) -> io::Result<()> {
    for entry in read_dir(post_dir)? {
        let path = entry?.path();
        if path.extension().unwrap().to_str() == Some("md") {
            let file = File::open(path)?;
            let md = BufReader::new(file);

            let (meta_toml, md) = parse_markdown(md)?;
            let meta = Metadata::new(&meta_toml)?;
        }
    }

    Ok(())
}
