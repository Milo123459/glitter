pub mod cli;
pub mod config;
pub mod get_and_parse;

use config::Arguments;

use crate::{
	cli::{cc, push, undo},
	get_and_parse::parse,
};

// this function will parse configuration from the get_and_parse file and pass it onto the cli
pub fn run(args: Arguments) -> anyhow::Result<()> {
	match args {
		Arguments::Push {
			arguments,
			base_cli,
		} => push(
			parse(&base_cli.rc_path)?,
			arguments,
			match_optional_bool(&base_cli.dry),
			base_cli.branch,
			match_optional_bool(&base_cli.nohost),
			match_optional_bool(&base_cli.raw),
			match_optional_bool(&base_cli.no_verify),
			base_cli.rc_path,
		),
		Arguments::Undo {
			how_many: _,
			base_cli,
		} => undo(match_optional_bool(&base_cli.dry)),
		Arguments::Cc {
			arguments,
			base_cli,
		} => cc(
			parse(&base_cli.rc_path)?,
			arguments,
			match_optional_bool(&base_cli.dry),
		),
	}
}

pub fn match_optional_bool(boolean: &Option<Option<bool>>) -> bool {
	match boolean {
		None => false,
		Some(None) => true,
		Some(Some(val)) => *val,
	}
}
