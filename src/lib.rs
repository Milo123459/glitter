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

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use crate::{config::Arguments, run};

//     we need a dry run thing in order to re-activate this

//     #[test]
//     fn runs_correctly() {
//         let args = Arguments {
//             action: "push".to_string(),
//             arguments: vec![
//                 "test".to_string(),
//                 "a".to_string(),
//                 "b".to_string(),
//                 "c".to_string(),
//             ],
//             rc_path: PathBuf::from(".glitterrc"),
//         };

//         run(args).unwrap();
//     }
// }
