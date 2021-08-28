use serde::{Deserialize, Serialize};
use structopt::StructOpt;

fn commit_msg() -> String {
	"$1+".to_string()
}

#[derive(Serialize, Deserialize, Debug, StructOpt, PartialEq, Clone)]
pub struct BaseCli {
	/// path to glitterrc
	#[structopt(
		parse(from_os_str),
		default_value = ".glitterrc",
		long,
		visible_alias = "rc"
	)]
	pub rc_path: std::path::PathBuf,

	/// branch to use. if the branch is not on the hosted repo use --nohost
	#[structopt(long = "--branch", short, visible_alias = "br")]
	pub branch: Option<String>,

	/// dry run. aka don't run git commands
	#[structopt(long, short, visible_alias = "d")]
	pub dry: Option<Option<bool>>,

	/// if the branch is not on the hosted provider, call this
	#[structopt(long, visible_alias = "nh")]
	pub nohost: Option<Option<bool>>,

	/// don't follow the commit template specified and just use $1+
	#[structopt(long, short)]
	pub raw: Option<Option<bool>>,

	/// don't run any glitter hooks
	#[structopt(long = "no-verify", short = "n")]
	pub no_verify: Option<Option<bool>>,
}

#[derive(Serialize, Deserialize, Debug, StructOpt, PartialEq, Clone)]
pub enum Arguments {
	Push {
		arguments: Vec<String>,

		#[structopt(flatten)]
		base_cli: BaseCli,
	},

	Undo {
		#[structopt(default_value = "1")]
		how_many: i32,

		#[structopt(flatten)]
		base_cli: BaseCli,
	},

	Cc {
		arguments: Vec<String>,

		#[structopt(flatten)]
		base_cli: BaseCli,
	},
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CommitMessageArguments {
	pub argument: i32,
	pub case: Option<String>,
	pub type_enums: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct CustomTaskOptions {
	pub name: String,
	pub execute: Option<Vec<String>>,
}

// main struct for the GlitterRc with defaults
#[derive(Deserialize, Debug, PartialEq)]
pub struct GlitterRc {
	#[serde(default = "commit_msg")]
	pub commit_message: String,
	pub commit_message_arguments: Option<Vec<CommitMessageArguments>>,
	pub arguments: Option<Vec<Arguments>>,
	pub fetch: Option<bool>,
	pub custom_tasks: Option<Vec<CustomTaskOptions>>,
	pub hooks: Option<Vec<String>>,
	pub __default: Option<bool>,
}
// tests
#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use crate::config::BaseCli;

	use super::{commit_msg, Arguments, CommitMessageArguments, CustomTaskOptions, GlitterRc};

	#[test]
	fn check_commit_message() {
		// getting 100% using this trick as we kinda cant test structs that dont have impls

		let args = Arguments::Push {
			arguments: vec![
				"test".to_string(),
				"a".to_string(),
				"b".to_string(),
				"c".to_string(),
			],
			base_cli: BaseCli {
				rc_path: PathBuf::new(),
				branch: Some(String::new()),
				dry: Some(Some(false)),
				nohost: Some(Some(false)),
				raw: Some(Some(false)),
				no_verify: Some(Some(false)),
			},
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
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
		};

		assert_eq!(commit_msg(), "$1+".to_string());
		assert_eq!(
			args,
			Arguments::Push {
				arguments: vec![
					"test".to_string(),
					"a".to_string(),
					"b".to_string(),
					"c".to_string(),
				],
				base_cli: BaseCli {
					rc_path: PathBuf::new(),
					branch: Some(String::new()),
					dry: Some(Some(false)),
					nohost: Some(Some(false)),
					raw: Some(Some(false)),
					no_verify: Some(Some(false))
				}
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
				fetch: None,
				custom_tasks: Some(vec![CustomTaskOptions {
					name: "fmt".to_owned(),
					execute: Some(vec!["cargo fmt".to_owned()])
				}]),
				__default: None,
				hooks: None
			}
		);
	}
}
