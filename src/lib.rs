use std::fs::File;
use std::io::{BufReader, BufRead, Error};
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

pub fn get_metadata(path: &str) -> Result<String, Error> {
  let file = File::open(path)?;
  let buffered = BufReader::new(file);

  let mut counter = 0;
  let mut metadata = String::new();

  for line in buffered.lines() {
    match line?.as_str() {
      "+++" => {
        counter += 1;
        if counter == 2 {
          break;
        }
      },
      s => metadata = format!("{}\n{}", metadata, s),
    };
  }

  Ok(metadata)
}