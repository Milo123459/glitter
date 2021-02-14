pub mod cli;
pub mod config;
pub mod get_and_parse;

use crate::cli::match_cmds;
use config::Arguments;

pub fn run(args: Arguments) -> anyhow::Result<()> {
    let config = get_and_parse::parse(&args.rc_path)?;
    match_cmds(args, config)?;

    Ok(())
}
