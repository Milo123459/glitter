use serde::{Deserialize, Serialize};
use structopt::StructOpt;

fn commit_msg() -> String {
    "$RAW_COMMIT_MSG".to_string()
}

#[derive(Serialize, Deserialize, Debug, StructOpt, PartialEq)]
pub struct Arguments {
    /// type of action. run the `action` action to see available actions.
    pub action: String,

    /// arguments to action
    pub arguments: Vec<String>,

    /// path to glitterrc
    #[structopt(
        parse(from_os_str),
        default_value = ".glitterrc",
        long,
        visible_alias = "rc"
    )]
    pub rc_path: std::path::PathBuf,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CommitMessageArguments {
    pub argument: i32,
    pub case: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GlitterRc {
    #[serde(default = "commit_msg")]
    pub commit_message: String,
    pub arguments: Option<Vec<Arguments>>,
    pub commit_message_arguments: Option<Vec<CommitMessageArguments>>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{commit_msg, Arguments, CommitMessageArguments, GlitterRc};

    #[test]
    fn check_commit_message() {
        // getting 100% using this trick as we kinda cant test structs that dont have impls

        let args = Arguments {
            action: "actions".to_string(),
            arguments: vec![
                "test".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
            rc_path: PathBuf::new(),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: Some(vec![CommitMessageArguments {
                argument: 1,
                case: Some("snake".to_string()),
            }]),
        };

        (args, config);

        assert_eq!(commit_msg(), "$RAW_COMMIT_MSG".to_string())
    }
}
