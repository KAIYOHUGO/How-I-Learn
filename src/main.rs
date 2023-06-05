use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, File},
    io::BufWriter,
};
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
struct List {
    programming: Item,
    math: Item,
    science: Item,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    #[serde(default)]
    video: Vec<Info>,
    #[serde(default)]
    article: Vec<Info>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    name: String,
    description: String,
    url: String,
}

fn main() -> Result<()> {
    let tera = Tera::new("template/*.md")?;
    let list: List = toml::from_str(&read_to_string("list.toml")?)?;
    let context = Context::from_serialize(list)?;
    let file = BufWriter::new(File::create("README.md")?);
    tera.render_to("readme.md", &context, file)?;
    Ok(())
}
