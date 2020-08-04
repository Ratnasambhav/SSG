use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn parse_metadata(path: &str) -> Result<String, Error> {
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