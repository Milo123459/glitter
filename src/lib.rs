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
// tests
#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use crate::{config::Arguments, run};

	#[test]
	fn runs_correctly() {
		let args = Arguments {
			action: "push".to_string(),
			arguments: vec![
				"feat".to_string(),
				"test".to_string(),
				"b".to_string(),
				"c".to_string(),
			],
			rc_path: PathBuf::from(".glitterrc"),
			dry: Some(Some(true)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
			yes: None,
		};

		run(args).unwrap();
	}
}
