use std::io::Error;

use crate::config::{Arguments, GlitterRc};
use crate::logger::Logger;

fn push(config: GlitterRc, args: Arguments) -> anyhow::Result<()> {
    if config.commit_message == "$RAW_COMMIT_MSG" {
        Logger::error("No template provided. A template has to be provided for Glitter to run the command push.");
        std::process::exit(1);
    }

    let splitted = config.commit_message.split('$').skip(1);

    let mut result = String::from(&config.commit_message);

    for val in splitted {
        if val.len() >= 2 && String::from(val.chars().nth(1).unwrap()) == String::from("+") {
            let idx = val.chars().nth(0).unwrap().to_digit(10).unwrap() - 1;
            let rest = &args.argument[idx as usize..];
            result = result.replace(
                &format!("${}+", String::from(val).split("").collect::<Vec<_>>()[1]),
                &rest.join(" "),
            );
        } else {
            let idx = val.split("").nth(1).unwrap().parse::<usize>().unwrap() - 1;

            if &args.argument.len() > &idx {
                return Err(anyhow::Error::new(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid Amount of parameters",
                )));
            }

            let val_ = &args.argument[idx];
            result = result.replace(
                &format!("${}", String::from(val).split("").collect::<Vec<_>>()[1]),
                &val_,
            );
        }
    }

    println!("{}", result);
    Ok(())
}

pub fn match_cmds(args: Arguments, config: GlitterRc) -> anyhow::Result<()> {
    let cmd = &args.case;
    match &*cmd.to_lowercase() {
        "push" => push(config, args),
        _ => Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid action. Can only be `push`",
        ))),
    }
}
