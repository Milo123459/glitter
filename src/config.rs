use serde::{Deserialize, Serialize};
use structopt::StructOpt;

fn commit_msg() -> String {
    "$RAW_COMMIT_MSG".to_string()
}

#[derive(Serialize, Deserialize, Debug, StructOpt, PartialEq)]
pub struct Arguments {
    /// type of action. Current options are: `push`
    pub action: String,

    /// arguments to action
    pub arguments: Vec<String>,

    /// path to glitterrc
    #[structopt(parse(from_os_str), default_value = ".glitterrc", long)]
    pub rc_path: std::path::PathBuf,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GlitterRc {
    #[serde(default = "commit_msg")]
    pub commit_message: String,
    pub commit_message_arguments: Option<Vec<Arguments>>,
}

#[cfg(test)]
mod tests {
    use super::commit_msg;

    #[test]
    fn check_commit_message() {
        assert_eq!(commit_msg(), "$RAW_COMMIT_MSG".to_string())
    }
}
