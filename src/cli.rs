use crate::config::{Arguments, GlitterRc};
use colored::*;
use fancy_regex::Regex;
use inflector::Inflector;
use std::io::{stdin, Error};
use std::process::Command;
// this is a macro that will return the patterns in match's
macro_rules! match_patterns {
    ($val:expr, $patterns_ident:ident, $($p:pat => $e:expr),*) => {
      let $patterns_ident = vec![$(stringify!($p)),*];
      match $val {
        $($p => $e),*
      }
    }
  }

fn get_commit_message(config: GlitterRc, args: Arguments) -> anyhow::Result<String> {
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
                    format!(
                        "Argument {0} was not provided. Argument {0} is a rest argument.",
                        String::from(val).split("").collect::<Vec<_>>()[1]
                    ),
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
                    format!(
                        "Argument {} was not provided.",
                        String::from(val).split("").collect::<Vec<_>>()[1]
                    ),
                )));
            }

            let mut val_ = (&args.arguments[idx]).clone();
            if let Some(ref args_) = config.commit_message_arguments {
                for arg in args_.iter().as_ref() {
                    if arg.argument == ((idx + 1) as i32) {
                        if let Some(v) = arg.case.as_deref() {
                            // we do this to prevent binding errors
                            let mut temp_val = val_.clone();
                            match v.to_lowercase().as_str() {
                                "lower" => temp_val = temp_val.to_lowercase(),
                                "upper" => temp_val = temp_val.to_uppercase(),
                                "snake" => temp_val = temp_val.to_snake_case(),
                                "screaming-snake" => temp_val = temp_val.to_screaming_snake_case(),
                                "kebab" => temp_val = temp_val.to_kebab_case(),
                                "train" => temp_val = temp_val.to_train_case(),
                                "sentence" => temp_val = temp_val.to_sentence_case(),
                                "title" => temp_val = temp_val.to_title_case(),
                                "pascal" => temp_val = temp_val.to_pascal_case(),
                                _ => println!("Found invalid case `{}`", v),
                            }
                            val_ = temp_val
                        }
                    }
                }
            }
            if let Some(ref args_) = config.commit_message_arguments {
                for arg in args_.iter().as_ref() {
                    if arg.argument == ((idx + 1) as i32) {
                        if let Some(v) = arg.type_enums.as_ref() {
                            if !v.contains(&val_.to_owned()) {
                                return Err(anyhow::Error::new(Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    format!(
                                        "Argument {} did not have a valid type enum.",
                                        String::from(val).split("").collect::<Vec<_>>()[1]
                                    ),
                                )));
                            }
                        }
                    }
                }
            }

            if let Some(ref args_) = config.commit_message_arguments {
                for arg in args_.iter().as_ref() {
                    if arg.argument == ((idx + 1) as i32) {
                        if let Some(v) = arg.type_enums.as_ref() {
                            if !v.contains(&val_.to_owned()) {
                                return Err(anyhow::Error::new(Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    format!(
                                        "Argument {} did not have a valid type enum.",
                                        String::from(val).split("").collect::<Vec<_>>()[1]
                                    ),
                                )));
                            }
                        }
                    }
                }
            }

            let captures = Regex::new(&format!(
                "\\${}(?!\\+)",
                String::from(val).split("").collect::<Vec<_>>()[1]
            ))?;

            let res = result.clone();

            // poor mans replace
            for _ in 0..captures.captures_iter(&res).count() {
                let res = result.clone();

                let capture = captures
                    // when we replace, the value changes, so we rerun the logic
                    .captures_iter(&res)
                    .collect::<Vec<_>>()
                    // we dont use the loop index as since the new value excludes the previous match we dont need to
                    .get(0)
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .get(0)
                    .unwrap();
                result.replace_range(capture.range(), &val_);
            }
        }
    }
    Ok(result)
}

pub fn push(config: GlitterRc, args: Arguments, dry: bool) -> anyhow::Result<()> {
    if dry {
        println!("{}", "Dry run. Won't execute git commands.".yellow());
    }

    let result = get_commit_message(config, args)?;
    if !dry {
        println!(
            "Commit message: {}. Is this correct? If correct please press enter, if not abort the process. (ctrl+c / cmd+c)",
            result.on_bright_black()
        );

        let mut temp = String::new();
        stdin().read_line(&mut temp)?;
    }

    println!("{} git add .", "$".green().bold());
    if !dry {
        Command::new("git").arg("add").arg(".").status()?;
    }

    println!(
        "{} git commit -m \"{}\"",
        "$".green().bold(),
        result.underline()
    );

    if !dry {
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&result)
            .status()?;
    }
    println!("{} git pull", "$".green().bold());
    if !dry {
        Command::new("git").arg("pull").status()?;
    }
    println!("{} git push", "$".green().bold());
    if !dry {
        Command::new("git").arg("push").status()?;
    }

    Ok(())
}

pub fn action(input: Vec<&str>) -> anyhow::Result<()> {
    // this will sanitize the vec in a sense
    // the input usually has \" \" around the value we want so we remove it
    // we also filter out _ from the vec
    let actions = input
        .into_iter()
        .filter_map(|x| x.strip_prefix('"')?.strip_suffix('"'))
        .collect::<Vec<_>>();
    println!("Actions available:\n{}", actions.join(", ").underline().bold());
    Ok(())
}

pub fn match_cmds(args: Arguments, config: GlitterRc) -> anyhow::Result<()> {
    let cmd = &args.action;
    let dry = args.clone().dry();
    // custom macro for the patterns command, see line 196-206
    match_patterns! { &*cmd.to_lowercase(), patterns,
        "push" => push(config, args, dry)?,
        "action" => action(patterns)?,
        "actions" => action(patterns)?,
        _ => return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid action. Try `--help`",
        )))
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cli::action;
    use crate::match_cmds;
    use std::path::PathBuf;

    use crate::config::{Arguments, CommitMessageArguments, GlitterRc};

    use super::get_commit_message;

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
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        assert_eq!(get_commit_message(config, args).unwrap(), "test(a): b c")
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
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+ : $2 | $1+".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        assert_eq!(
            get_commit_message(config, args).unwrap(),
            "test(a): b c : a | test a b c"
        )
    }

    #[test]
    fn less_than_needed_args() {
        let args = Arguments {
            action: "push".to_string(),
            arguments: vec!["test".to_string(), "a".to_string()],
            rc_path: PathBuf::new(),
            dry: Some(Some(false)),
        };

        let args_2 = Arguments {
            action: "push".to_string(),
            arguments: vec!["test".to_string()],
            rc_path: PathBuf::new(),
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        let config_2 = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        assert!(get_commit_message(config, args).is_err());
        assert!(get_commit_message(config_2, args_2).is_err());
    }

    #[test]
    fn no_commit_message_format() {
        let args = Arguments {
            action: "push".to_string(),
            arguments: vec!["test".to_string(), "a".to_string()],
            rc_path: PathBuf::new(),
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            // "$RAW_COMMIT_MSG" is the default
            commit_message: "$RAW_COMMIT_MSG".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        assert!(get_commit_message(config, args).is_err())
    }

    #[test]
    fn commit_message_arguments() {
        let args = Arguments {
            action: "push".to_string(),
            arguments: vec!["feat".to_string(), "test".to_string(), "tests".to_string()],
            rc_path: PathBuf::new(),
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            commit_message: "$1: $2: $3+".to_string(),
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
        };

        assert_eq!(
            get_commit_message(config, args).unwrap(),
            "feat: test: tests"
        )
    }

    #[test]
    fn test_action() {
        assert!(action(vec!["test"]).is_ok())
    }

    #[test]
    fn matching_cmds() {
        let args = Arguments {
            action: "action".to_string(),
            arguments: vec![
                "test".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
            rc_path: PathBuf::new(),
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        assert!(match_cmds(args, config).is_ok());

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
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        assert!(match_cmds(args, config).is_ok());

        let args = Arguments {
            action: "fasdafsfsa".to_string(),
            arguments: vec![
                "test".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
            rc_path: PathBuf::new(),
            dry: Some(Some(false)),
        };

        let config = GlitterRc {
            commit_message: "$1($2): $3+".to_string(),
            arguments: None,
            commit_message_arguments: None,
        };

        assert!(match_cmds(args, config).is_err());
    }
}
