use serde::{Deserialize, Serialize};
use structopt::StructOpt;

fn commit_msg() -> String {
	"$1+".to_string()
}

#[derive(Serialize, Deserialize, Debug, StructOpt, Eq, PartialEq, Clone)]
pub struct Arguments {
	/// type of action. run the `action` / `actions` action to see available actions.
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

	/// dry run. aka don't actually run the commands
	#[structopt(long, short, visible_alias = "d")]
	pub(crate) dry: Option<Option<bool>>,

	/// don't follow the commit template specified and just use $1+
	#[structopt(long, short)]
	pub(crate) raw: Option<Option<bool>>,

	/// don't run any glitter hooks
	#[structopt(long = "no-verify", short = "n")]
	pub(crate) no_verify: Option<Option<bool>>,

	/// verbose mode: log the output of all commands run
	#[structopt(long = "verbose", short = "v")]
	pub(crate) verbose: Option<Option<bool>>,

	/// don't run `git add .`
	#[structopt(long = "no-add")]
	pub(crate) no_add: Option<Option<bool>>,
}

pub struct VerboseResponse {
	pub provided: bool,
	pub value: bool,
}

// for the usage shorthand things, ie, without a value
impl Arguments {
	pub fn dry(&self) -> bool {
		match self.dry {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}
	pub fn raw(&self) -> bool {
		match self.raw {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}
	pub fn no_verify(&self) -> bool {
		match self.no_verify {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}
	pub fn verbose(&self) -> VerboseResponse {
		match self.verbose {
			None => VerboseResponse {
				provided: false,
				value: false,
			},
			Some(None) => VerboseResponse {
				provided: true,
				value: true,
			},
			Some(Some(a)) => VerboseResponse {
				provided: true,
				value: a,
			},
		}
	}
	pub fn no_add(&self) -> bool {
		match self.no_add {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct CommitMessageArguments {
	pub argument: i32,
	pub case: Option<String>,
	pub type_enums: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CustomTaskOptions {
	pub name: String,
	pub execute: Option<Vec<String>>,
}

// main struct for the GlitterRc with defaults
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct GlitterRc {
	#[serde(default = "commit_msg")]
	pub commit_message: String,
	pub arguments: Option<Vec<Arguments>>,
	pub commit_message_arguments: Option<Vec<CommitMessageArguments>>,
	pub fetch: Option<bool>,
	pub custom_tasks: Option<Vec<CustomTaskOptions>>,
	pub hooks: Option<Vec<String>>,
	pub __default: Option<bool>, // this really shouldn't be provided by a user, but it's here for backwards compatibility
	pub verbose: Option<bool>,
}
// tests
#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use super::{commit_msg, Arguments, CommitMessageArguments, CustomTaskOptions, GlitterRc};

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
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
			no_add: Some(Some(false)),
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
			verbose: None,
		};

		assert_eq!(commit_msg(), "$1+".to_string());
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
				raw: Some(Some(false)),
				no_verify: Some(Some(false)),
				verbose: Some(Some(false)),
				no_add: Some(Some(false)),
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
				hooks: None,
				verbose: None
			}
		);
	}
}
