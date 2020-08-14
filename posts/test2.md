+++
title = "Test 2"
date = "August 4th, 2020"

[meta]
description = "A markdown file for tests during development."
keywords = "markdown, html, rust, static site generator"
+++

# Test

There are two common formats that look very similar but are actually different in some very specific ways. And a third which is very different.

## YAML Front Matter

The Jekyll static site generator popularized YAML front matter which is deliminated by YAML section markers. Yes, the dashes are actually part of the YAML syntax. And the metadata is defined using any valid YAML syntax. 

Note that YAML front matter is not parsed by the Markdown parser, but is removed prior to parsing by Jekyll (or whatever tool you're using) and could actually be used to request a different parser than the default Markdown parser for that page (I don't recall if Jekyll does that, but I have seen some tools which do).

## MultiMarkdown Metadata

The older and simpler MultiMarkdown Metadata is actually incorporated into a few Markdown parsers. While it has more recently been updated to optionally support YAML deliminators, traditionally, the metadata ends and the Markdown document begins upon the first blank line (if the first line was blank, then no metadata). And while the syntax looks very similar to YAML, only key-value pairs are supported with no implied types.

The MultiMarkdown parser includes a bunch of additional options which are unique to that parser, but the key-value metadata is used across multiple parsers. Unfortunately, I have never seen any two which behaved exactly the same. Without the Markdown rules defining such a format everyone has done their own slightly different interpretation resulting in a lot of variety.

The one thing that is more common is the support for YAML deliminators and basic key-value definitions.

## Code

```rust
use serde::Deserialize;

use toml::de::Error;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn main() -> Result<(), Error> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}
```