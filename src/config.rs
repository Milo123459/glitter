use serde::{Deserialize, Serialize};
use structopt::StructOpt;

fn commit_msg() -> String {
    "$RAW_COMMIT_MSG".to_string()
}

#[derive(Serialize, Deserialize, Debug, StructOpt, PartialEq, Clone)]
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

    #[structopt(long, short)]
    pub(crate) dry: Option<Option<bool>>,
}
// for the usage of --dry (shorthand, ie, without a value)
impl Arguments {
    pub fn dry(&self) -> bool {
        match self.dry {
            None => false,
            Some(None) => true,
            Some(Some(a)) => a,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CommitMessageArguments {
    pub argument: i32,
    pub case: Option<String>,
    pub type_enums: Option<Vec<String>>,
}
// main struct for the GlitterRc with defaults
#[derive(Deserialize, Debug, PartialEq)]
pub struct GlitterRc {
    #[serde(default = "commit_msg")]
    pub commit_message: String,
    pub arguments: Option<Vec<Arguments>>,
    pub commit_message_arguments: Option<Vec<CommitMessageArguments>>,
    pub fetch: Option<bool>
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
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: Some(vec![CommitMessageArguments {
                argument: 1,
                case: Some("snake".to_string()),
                type_enums: Some(vec![
                    "fix".to_owned(),
                    "feat".to_owned(),
                    "chore".to_owned(),
                ]),
            }]),
            fetch: None
        };

        assert_eq!(commit_msg(), "$RAW_COMMIT_MSG".to_string());
        assert_eq!(
            args,
            Arguments {
                action: "actions".to_string(),
                arguments: vec![
                    "test".to_string(),
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                ],
                rc_path: PathBuf::new(),
                dry: Some(Some(false)),
            }
        );
        assert_eq!(
            config,
            GlitterRc {
                commit_message: "$1($2): $3+".to_string(),
                arguments: None,
                commit_message_arguments: Some(vec![CommitMessageArguments {
                    argument: 1,
                    case: Some("snake".to_string()),
                    type_enums: Some(vec![
                        "fix".to_owned(),
                        "feat".to_owned(),
                        "chore".to_owned()
                    ])
                }]),
                fetch: None
            }
        );
    }
}
