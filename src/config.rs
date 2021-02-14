use serde::{Deserialize, Serialize};
use structopt::StructOpt;

fn commit_msg() -> String {
    String::from("$RAW_COMMIT_MSG")
}

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct Arguments {
    /// type
    pub case: String,
    /// arguments
    pub argument: Vec<String>,
    /// path to glitterrc, default is ".glitterrc"
    #[structopt(parse(from_os_str), default_value = ".glitterrc", long)]
    pub rc_path: std::path::PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct GlitterRc {
    #[serde(default = "commit_msg")]
    pub commit_message: String,
    pub commit_message_arguments: Option<Vec<Arguments>>,
}
