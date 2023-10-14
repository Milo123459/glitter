pub mod cli;
pub mod config;
pub mod get_and_parse;
use crate::cli::match_cmds;
use config::Arguments;

// this function will parse configuration from the get_and_parse file and pass it onto the cli
pub fn run(args: Arguments) -> anyhow::Result<()> {
	let config = get_and_parse::parse(&args.rc_path)?;
	match_cmds(args, config)?;

	Ok(())
}
