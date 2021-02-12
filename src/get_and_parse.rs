use std::path::Path;
use std::fs::File;
use serde::{Deserialize, Serialize};
use std::result::Result;
use crate::logger;

#[derive(Serialize, Deserialize)]
pub struct GlitterRc {
    commit_message: String
}

pub fn get_and_parse() -> Result<GlitterRc, serde_json::Error> {
    let json_file_path = Path::new("./.glitterrc");

    let file = match File::open(json_file_path) {
        Err(_) => logger::error("File didn't open."),
        _ => println!("something")
    };

    let config: GlitterRc = serde_json::from_reader(file).expect("Well that didn't work..");
    return config;
}

// genius
// :wesmart: wait a minute wont this not work if the file doesnt exist wait lets see
// it should since we add the .expect() basically handles the error dms