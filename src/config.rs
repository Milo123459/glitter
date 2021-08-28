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
