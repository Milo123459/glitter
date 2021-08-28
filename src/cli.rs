use crate::config::{Arguments, BaseCli, CustomTaskOptions, GlitterRc};
use colored::*;
use fancy_regex::Regex;
use inflector::Inflector;
use std::io::{stdin, Error};
use std::path::{Path, PathBuf};
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

fn get_commit_message(config: &GlitterRc, args: Vec<String>) -> anyhow::Result<String> {
	let splitted = config.commit_message.split('$').skip(1);

	let mut result = String::from(&config.commit_message);

	for val in splitted {
		if val.len() >= 2 && String::from(val.chars().nth(1).unwrap()) == *"+" {
			let idx = val.chars().next().unwrap().to_digit(10).unwrap() - 1;
			let rest = &args[idx as usize..];

			if rest.is_empty() {
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

			if args.len() <= idx {
				return Err(anyhow::Error::new(Error::new(
					std::io::ErrorKind::InvalidInput,
					format!(
						"Argument {} was not provided.",
						String::from(val).split("").collect::<Vec<_>>()[1]
					),
				)));
			}

			let mut val_ = (&args[idx]).clone();
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
						if let Some(valid_type_enums) = arg.type_enums.as_ref() {
							if !valid_type_enums.contains(&val_.to_owned()) {
								return Err(anyhow::Error::new(Error::new(
									std::io::ErrorKind::InvalidInput,
									format!(
										"Argument {} did not have a valid type enum. Valid type enums are {}",
										String::from(val).split("").collect::<Vec<_>>()[1],
                                        valid_type_enums.join(", ").red().to_string()
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
						if let Some(valid_type_enums) = arg.type_enums.as_ref() {
							if !valid_type_enums.contains(&val_.to_owned()) {
								return Err(anyhow::Error::new(Error::new(
									std::io::ErrorKind::InvalidInput,
									format!(
										"Argument {} did not have a valid type enum. Valid type enums are {}",
										String::from(val).split("").collect::<Vec<_>>()[1],
                                        valid_type_enums.join(", ").red().to_string()
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

pub fn push(
	config: GlitterRc,
	args: Vec<String>,
	dry: bool,
	branch: Option<String>,
	nohost: bool,
	raw: bool,
	no_verify: bool,
	rc_path: PathBuf,
) -> anyhow::Result<()> {
	let is_git_folder = Path::new(".git").exists();
	if !is_git_folder {
		return Err(anyhow::Error::new(Error::new(
			std::io::ErrorKind::InvalidInput,
			format!(
				"{} This is not a git repository.",
				"Fatal".red().to_string()
			),
		)));
	}
	let current_branch = String::from_utf8(
		Command::new("git")
			.arg("branch")
			.arg("--show-current")
			.output()
			.unwrap()
			.stdout,
	)
	.unwrap();
	if dry {
		println!(
			"{} {} {}",
			"Dry run.".yellow(),
			"Won't".yellow().underline(),
			"execute git commands or glitter hooks.".yellow()
		);
	}
	if config.hooks.is_some() && config.hooks.clone().unwrap().is_empty() && no_verify {
		println!(
			"{} Redundant usage of {}. There are no Glitter hooks in this project",
			"Warn".yellow(),
			"no-verify".bold()
		)
	}
	let mut _result = String::new();
	if !raw {
		_result = get_commit_message(&config, args)?;
	} else {
		let raw_args = args.clone();
		let raw_branch = branch.clone();
		_result = get_commit_message(
			&GlitterRc {
				commit_message: "$1+".to_owned(),
				arguments: Some(vec![Arguments::Push {
					arguments: args,
					base_cli: BaseCli {
						rc_path,
						branch: raw_branch,
						dry: Some(Some(dry)),
						nohost: Some(Some(nohost)),
						raw: Some(Some(raw)),
						no_verify: Some(Some(no_verify)),
					},
				}]),
				commit_message_arguments: None,
				fetch: None,
				custom_tasks: None,
				__default: None,
				hooks: None,
			},
			raw_args,
		)?
	}
	if !dry {
		println!(
            "Commit message: {}. Is this correct? If correct please press enter, if not abort the process. (ctrl+c / cmd+c)",
            format!("{}{}{}", "`".green(), _result.underline().green(), "`".green())
        );
		// if they abort the process (cmd+c / ctrl+c), this will error and stop
		// if they press enter the command will then start executing git commands
		let mut temp = String::new();
		stdin().read_line(&mut temp)?;
	}
	if let Some(fetch) = config.fetch {
		if fetch {
			println!("{} git fetch", "$".green().bold());
			if !dry {
				Command::new("git").arg("fetch").status()?;
			}

			println!("{}", "".normal().clear().to_string());
		}
	}
	// glitter hooks
	if config.custom_tasks.is_some()
		&& config.hooks.is_some()
		&& !config.hooks.clone().unwrap().is_empty()
	{
		let tasks = &config.custom_tasks.unwrap();
		let task_names = &tasks
			.iter()
			.map(|task| task.clone().name)
			.collect::<Vec<String>>();
		let hooks = &config.hooks.unwrap();
		for hook in hooks.clone() {
			if !task_names.contains(&hook) {
				println!("{} Couldn't find the custom task `{}`", "Fatal".red(), hook);
				std::process::exit(1);
			}
			let custom_task = &tasks.iter().find(|task| task.name == hook);
			if let Some(task) = custom_task {
				for cmd in task.execute.clone().unwrap() {
					if !no_verify {
						println!("{} {}", "$".green().bold(), cmd);
					}
					if !dry && !no_verify {
						let splitted = cmd.split(' ').collect::<Vec<&str>>();
						let command = which::which(splitted.first().unwrap());
						if command.is_err() {
							println!(
								"{} Cannot find binary `{}`",
								"Fatal".red(),
								&&(*(*splitted.first().unwrap()))
							);
							std::process::exit(1);
						}
						let status = Command::new(command.unwrap())
							.args(&splitted[1..])
							.status()
							.unwrap();
						if !&status.clone().success() {
							std::process::exit(1);
						}
					}
				}
			} else {
				println!("{} Couldn't find the custom task `{}`", "Fatal".red(), hook);
				std::process::exit(1);
			}
		}
	}
	println!("{} git add .", "$".green().bold());
	if !dry {
		Command::new("git").arg("add").arg(".").status()?;
	}
	println!(
		"{} git commit -m {}",
		"$".green().bold(),
		format!(
			"{}{}{}",
			"`".green(),
			_result.underline().green(),
			"`".green()
		)
	);

	if !dry {
		Command::new("git")
			.arg("commit")
			.arg("-m")
			.arg(&_result)
			.status()?;
	}
	if !nohost {
		if let Some(br) = &branch {
			println!(
				"{} git pull origin {}",
				"$".green().bold(),
				br.to_string().green().underline()
			);
		} else {
			println!(
				"{} git pull origin {}",
				"$".green().bold(),
				current_branch
					.split('\n')
					.next()
					.unwrap()
					.green()
					.underline()
			)
		}
	}
	if !dry && !nohost {
		if let Some(br) = &branch {
			Command::new("git")
				.arg("pull")
				.arg("origin")
				.arg(br.to_lowercase())
				.status()?;
		}
		Command::new("git")
			.arg("pull")
			.arg("origin")
			.arg(current_branch.split('\n').next().unwrap())
			.status()?;
	}
	if let Some(br) = &branch {
		println!(
			"{}{} git push origin {}",
			"".clear().to_string(),
			"$".green().bold(),
			br.green().underline()
		);
	} else {
		println!(
			"{} git push origin {}",
			"$".green().bold(),
			current_branch
				.split('\n')
				.next()
				.unwrap()
				.green()
				.underline()
		);
	}
	if !dry {
		if let Some(br) = &branch {
			Command::new("git")
				.arg("push")
				.arg("origin")
				.arg(br.to_lowercase())
				.status()?;
		} else {
			Command::new("git")
				.arg("push")
				.arg("origin")
				.arg(current_branch.split('\n').next().unwrap())
				.status()?;
		}
	}

	Ok(())
}

pub fn action(input: Vec<&str>) -> anyhow::Result<()> {
	// this will sanitize the vec in a sense
	// the input has \" \" around the value we want so we remove it
	// we also filter out _ from the vec
	let actions = input
		.into_iter()
		.filter_map(|x| x.strip_prefix('"')?.strip_suffix('"'))
		.collect::<Vec<_>>();
	// log a nice message displaying all the actions
	println!(
		"Actions available:\n{}",
		actions.join(", ").underline().bold()
	);
	Ok(())
}

pub fn cc(config: GlitterRc, args: Vec<String>, dry: bool) -> anyhow::Result<()> {
	if args.first().is_some() {
		match_patterns! { &*args.first().unwrap().to_lowercase(), patterns,
			"list" => {
				let mut cmds: Vec<CustomTaskOptions> = vec![];
				if let Some(v) = config.custom_tasks {
					cmds = v;
				}
				let cmd = cmds.into_iter().map(|x| x.name).collect::<Vec<String>>();
				println!(
					"Custom tasks specified:\n{}",
					cmd.join(", ").underline().bold()
				);
			},

			"help" => {
				let mut cmds: Vec<CustomTaskOptions> = vec![];
				if let Some(v) = config.custom_tasks {
					cmds = v;
				}
				let actions = patterns
				.into_iter()
				.filter_map(|x| x.strip_prefix('"')?.strip_suffix('"'))
				.collect::<Vec<_>>();
				let cmd = cmds.into_iter().map(|x| x.name).collect::<Vec<String>>();
				println!(
					"Runnable commands:\n{}\nCustom tasks specified:\n{}",
					actions.join(", ").underline().bold(),
					cmd.join(", ").underline().bold()
				);
			},
			_ => {
				let mut cmds: Vec<CustomTaskOptions> = vec![];
				let mut exec_cmds: Vec<CustomTaskOptions> = vec![];
				if let Some(v) = config.custom_tasks {
					cmds = v.clone();
					exec_cmds = v;
				};
				if dry {
					println!(
						"{} {} {}",
						"Dry run.".yellow(),
						"Won't".yellow().underline(),
						"execute commands specified.".yellow()
					);
				}
				let cmd = cmds.into_iter().map(|x| x.name).collect::<Vec<String>>();
				if cmd.into_iter().any(|
					s| s == args.first().unwrap().to_lowercase()) {
					let exec = exec_cmds.into_iter().filter(|x| x.name == args.first().unwrap().to_lowercase()).map(|x| x.execute);
					 for task in exec {
						let e = task.to_owned().unwrap();
						// because it is a vec, we must do a for loop to get each command  & execute if dry is false
						for cmd in e {
							let splitted = cmd.split(' ').collect::<Vec<&str>>();
							println!("{} {}", "$".green().bold(), cmd);
							let command = which::which(splitted.first().unwrap());
							if command.is_err() {
								println!("{} Cannot find binary `{}`", "Fatal".red(), &&(*(*splitted.first().unwrap())));
								std::process::exit(1);
							}
							if !dry {
								Command::new(command.unwrap()).args(&splitted[1..]).envs(std::env::vars()).status()?;
							}

						}
					};
				} else {
					return Err(anyhow::Error::new(Error::new(
						std::io::ErrorKind::InvalidInput,
						"That is not a valid custom command / sub command.",
					)));
				};
			}
		};
	} else {
		println!("Try `cc help`")
	};
	Ok(())
}

pub fn undo(dry: bool) -> anyhow::Result<()> {
	if dry {
		println!(
			"{} {} {}",
			"Dry run.".yellow(),
			"Won't".yellow().underline(),
			"execute git commands.".yellow()
		);
	}
	println!("{} git reset --soft HEAD~1", "$".green().bold());
	if !dry {
		Command::new("git")
			.arg("reset")
			.arg("--soft")
			.arg("HEAD~1")
			.status()?;
	}
	Ok(())
}
