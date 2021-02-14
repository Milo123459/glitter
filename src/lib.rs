pub mod cli;
pub mod config;
pub mod get_and_parse;
pub mod logger;

use crate::cli::match_cmds;
use config::Arguments;
use structopt::StructOpt;

pub fn run() -> anyhow::Result<()> {
    let args = Arguments::from_args();

    let config = get_and_parse::parse(&args.rc_path)?;
    match_cmds(args, config)?;

    Ok(())
}
