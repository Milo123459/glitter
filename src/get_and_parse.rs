use std::path::Path;
use std::fs::File;
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
    case: String,
    argument: i32
}
#[derive(Deserialize, Debug)]
pub struct GlitterRc {
    #[serde(default = "commit_msg")]
    commit_message: String,
    #[serde(default = "commit_msg_arguments")]
    commit_message_arguments: Vec<Arguments>
}

pub fn get_and_parse() -> Result<GlitterRc, serde_json::Error> {
    let json_file_path = Path::new("./.glitterrc");

    let file = File::open(json_file_path).expect("Error opening file");

    serde_json::from_reader(file)
}

fn commit_msg() -> String {
    return String::from("$RAW_COMMIT_MSG")
}

fn commit_msg_arguments() -> Vec<Arguments> {
    let vec = vec![Arguments { case: "pascal".to_owned(), argument: 0 }];
    return vec;
}