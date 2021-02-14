use std::io::Error;

use onig::Regex;

use crate::config::{Arguments, GlitterRc};

fn push(config: GlitterRc, args: Arguments) -> anyhow::Result<String> {
    if config.commit_message == "$RAW_COMMIT_MSG" {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "No template provided. A template has to be provided for Glitter to run the command push.",
        )));
    }

    let splitted = config.commit_message.split('$').skip(1);

    let mut result = String::from(&config.commit_message);

    for val in splitted {
        if val.len() >= 2 && String::from(val.chars().nth(1).unwrap()) == String::from("+") {
            let idx = val.chars().nth(0).unwrap().to_digit(10).unwrap() - 1;
            let rest = &args.arguments[idx as usize..];

            if rest.len() == 0 {
                return Err(anyhow::Error::new(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Argument ${} was not provided", val),
                )));
            }

            result = result.replace(
                &format!("${}+", String::from(val).split("").collect::<Vec<_>>()[1]),
                &rest.join(" "),
            );
        } else {
            let idx = val.split("").nth(1).unwrap().parse::<usize>().unwrap() - 1;

            if &args.arguments.len() <= &idx {
                return Err(anyhow::Error::new(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid Amount of parameters",
                )));
            }

            let val_ = &*args.arguments[idx];
            result = Regex::new(&format!(
                "\\${}(?!@)",
                String::from(val).split("").collect::<Vec<_>>()[1]
            ))?
            .replace(&result, val_)
        }
    }

    println!("{}", result);
    Ok(result)
}

pub fn match_cmds(args: Arguments, config: GlitterRc) -> anyhow::Result<String> {
    let cmd = &args.action;
    match &*cmd.to_lowercase() {
        "push" => push(config, args),
        _ => Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid action. Can only be `push`",
        ))),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::{Arguments, GlitterRc};

    use super::push;

    #[test]
    fn basic() {
        let args = Arguments {
            action: "push".to_string(),
            arguments: vec![
                "test".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
            rc_path: PathBuf::new(),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            commit_message_arguments: None,
        };

        assert_eq!(push(config, args).unwrap(), "test(a): b c")
    }

    #[test]
    fn reuse_arguments() {
        let args = Arguments {
            action: "push".to_string(),
            arguments: vec![
                "test".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
            rc_path: PathBuf::new(),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+ : $2 | $1+".to_string(),
            commit_message_arguments: None,
        };

        assert_eq!(push(config, args).unwrap(), "test(a): b c : a | test a b c")
    }

    #[test]
    fn less_than_needed_args() {
        let args = Arguments {
            action: "push".to_string(),
            arguments: vec!["test".to_string(), "a".to_string()],
            rc_path: PathBuf::new(),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            commit_message_arguments: None,
        };

        assert_eq!(push(config, args).is_err(), true)
    }
}
