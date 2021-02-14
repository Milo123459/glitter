use crate::config::GlitterRc;
use anyhow::Context;
use std::fs::File;
use std::path::PathBuf;

pub fn get_and_parse(path: &PathBuf) -> anyhow::Result<GlitterRc> {
    let file = File::open(path.as_path())
        .with_context(|| format!("Could not read file `{}`", path.as_path().to_str().unwrap()))?;

    match serde_json::from_reader(file) {
        Ok(json) => Ok(json),
        Err(err) => Err(anyhow::Error::new(err)),
    }
}
