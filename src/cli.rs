use crate::config::{Arguments, CustomTaskOptions, GlitterRc};
use colored::*;
use fancy_regex::Regex;
use inflector::Inflector;
use std::io::{stdin, Error};
use std::path::Path;
use std::process::Command;
use terminal_spinners::{SpinnerBuilder, DOTS};

// this is a macro that will return the patterns in match's
macro_rules! match_patterns {
    ($val:expr, $patterns_ident:ident, $($p:pat => $e:expr),*) => {
      let $patterns_ident = vec![$(stringify!($p)),*];
      match $val {
        $($p => $e),*
      }
    }
  }

fn get_commit_message(config: &GlitterRc, args: &Arguments) -> anyhow::Result<String> {
	let splitted = config.commit_message.split('$').skip(1);

	let mut result = String::from(&config.commit_message);

	for val in splitted {
		if val.len() >= 2 && String::from(val.chars().nth(1).unwrap()) == *"+" {
			let idx = val.chars().next().unwrap().to_digit(10).unwrap() - 1;
			let rest = &args.arguments[idx as usize..];

			if rest.is_empty() {
				return Err(anyhow::Error::new(Error::new(
					std::io::ErrorKind::InvalidInput,
					format!(
						"Argument {0} was not provided. Argument {0} is a rest argument.",
						val.split("").collect::<Vec<_>>()[1]
					),
				)));
			}

			result = result.replace(
				&format!("${}+", val.split("").collect::<Vec<_>>()[1]),
				&rest.join(" "),
			);
		} else {
			let idx = val.split("").nth(1).unwrap().parse::<usize>().unwrap() - 1;

			if args.arguments.len() <= idx {
				return Err(anyhow::Error::new(Error::new(
					std::io::ErrorKind::InvalidInput,
					format!(
						"Argument {} was not provided.",
						val.split("").collect::<Vec<_>>()[1]
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
						if let Some(valid_type_enums) = arg.type_enums.as_ref() {
							if !valid_type_enums.contains(&val_.to_owned()) {
								return Err(anyhow::Error::new(Error::new(
									std::io::ErrorKind::InvalidInput,
									format!(
										"Argument {} did not have a valid type enum. Valid type enums are {}",
										val.split("").collect::<Vec<_>>()[1],
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
										val.split("").collect::<Vec<_>>()[1],
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
				val.split("").collect::<Vec<_>>()[1]
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

#[allow(clippy::too_many_arguments)]
pub fn push(
	config: GlitterRc,
	args: Arguments,
	dry: bool,
	branch: Option<String>,
	nohost: bool,
	raw: bool,
	no_verify: bool,
	verbose: bool,
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
		_result = get_commit_message(&config, &args)?;
	} else {
		let raw_args = args.clone();
		_result = get_commit_message(
			&GlitterRc {
				commit_message: "$1+".to_owned(),
				arguments: Some(vec![args]),
				commit_message_arguments: None,
				fetch: None,
				custom_tasks: None,
				__default: None,
				hooks: None,
				verbose: None,
			},
			&raw_args,
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
			run_cmd("git", vec!["fetch"], dry, verbose, None);
		}
	} // glitter hooks
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
						let splitted = cmd.split(' ').collect::<Vec<&str>>();
						run_cmd(
							splitted.first().unwrap(),
							splitted[1..].to_vec(),
							dry || no_verify,
							verbose,
							None,
						);
					}
				}
			} else {
				println!("{} Couldn't find the custom task `{}`", "Fatal".red(), hook);
				std::process::exit(1);
			}
		}
	}
	run_cmd("git", vec!["add", "."], dry, verbose, None);

	run_cmd(
		"git",
		vec!["commit", "-m", &_result],
		dry,
		verbose,
		Some(&*format!(
			"git commit -m {}",
			format!("{}{}{0}", "`".green(), _result.underline().green())
		)),
	);
	let mut args = vec!["pull", "origin"];
	if !nohost {
		if let Some(br) = &branch {
			args.push(&*br);
		}
		args.push(current_branch.split('\n').next().unwrap());
	}
	run_cmd(
		"git",
		args.clone(),
		dry,
		verbose,
		Some(&*format!(
			"git pull origin {}",
			args.clone().last().unwrap().green().underline()
		)),
	);

	let mut args = vec!["push", "origin"];
	if !nohost {
		if let Some(br) = &branch {
			args.push(&*br);
		}
		args.push(current_branch.split('\n').next().unwrap());
	}
	run_cmd(
		"git",
		args.clone(),
		dry,
		verbose,
		Some(&*format!(
			"git push origin {}",
			args.clone().last().unwrap().green().underline()
		)),
	);

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

pub fn cc(config: GlitterRc, args: Arguments, dry: bool, verbose: bool) -> anyhow::Result<()> {
	if args.arguments.first().is_some() {
		match_patterns! { &*args.arguments.first().unwrap().to_lowercase(), patterns,
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
					"Runnable commands:\n{}\nCustom tasks specified:\n{}\nIf the output of custom tasks is valuable to you, please provide the -v flag when running.",
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

				if cmds.into_iter().map(|x| x.name).any(|
					s| s == args.arguments.first().unwrap().to_lowercase()) {
					let exec = exec_cmds.into_iter().filter(|x| x.name == args.arguments.first().unwrap().to_lowercase()).map(|x| x.execute);
					 for task in exec {
						let e = task.to_owned().unwrap();
						// because it is a vec, we must do a for loop to get each command  & execute if dry is false
						for cmd in e {
							let splitted = cmd.split(' ').collect::<Vec<&str>>();
							let command = which::which(splitted.first().unwrap());
							if command.is_err() {
								println!("{} Cannot find binary `{}`", "Fatal".red(), splitted.first().unwrap());
								std::process::exit(1);
							}
							if !dry {
								run_cmd(
								splitted.first().unwrap(),
								splitted[1..].to_vec(),
								dry,
								verbose,
								None,
							);
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

pub fn undo(dry: bool, verbose: bool) -> anyhow::Result<()> {
	if dry {
		println!(
			"{} {} {}",
			"Dry run.".yellow(),
			"Won't".yellow().underline(),
			"execute git commands.".yellow()
		);
	}
	run_cmd("git", vec!["reset", "-soft", "HEAD~1"], dry, verbose, None);
	Ok(())
}
// this is the function behind matching commands (as in actions)
pub fn match_cmds(args: Arguments, config: GlitterRc) -> anyhow::Result<()> {
	let cmd = &args.action;
	let dry = args.dry();
	let nohost = args.nohost();
	let raw_mode = args.raw();
	let is_default = config.__default.is_some();
	let no_verify = args.no_verify();
	let verbose = args.verbose();
	let branch = args.clone().branch;
	let verbose = if verbose.provided {
		verbose.value
	} else {
		config.verbose.unwrap_or(false)
	};

	if is_default {
		println!("{} Using default config", "Warn".yellow())
	}
	// custom macro for the patterns command
	match_patterns! { &*cmd.to_lowercase(), patterns,
		"push" => push(config, args, dry, branch, nohost, raw_mode, no_verify, verbose)?,
		"action" => action(patterns)?,
		"actions" => action(patterns)?,
		"cc" => cc(config, args, dry, verbose)?,
		"undo" => undo(dry, verbose)?,
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

				if cmds.into_iter().map(|x| x.name).any(|
					s| s == args.action.to_lowercase()) {
					let exec = exec_cmds.into_iter().filter(|x| x.name == args.action.to_lowercase()).map(|x| x.execute);
					 for task in exec {
						let e = task.to_owned().unwrap();
						// because it is a vec, we must do a for loop to get each command & execute if dry is false
						for cmd in e {
							let splitted = cmd.split(' ').collect::<Vec<&str>>();
							let command = which::which(splitted.first().unwrap());
							if command.is_err() {
								println!("{} Cannot find binary `{}`", "Fatal".red(), splitted.first().unwrap());
								std::process::exit(1);
							}
							if !dry {
								run_cmd(
									splitted.first().unwrap(),
									splitted[1..].to_vec(),
									dry,
									verbose,
									None,
								);
							}

						}
					};
				} else {
					return Err(anyhow::Error::new(Error::new(
						std::io::ErrorKind::InvalidInput,
						"This is not a valid action or custom command.",
					)));
				};

	}
	};
	Ok(())
}

fn run_cmd(
	command_name: &str,
	args: Vec<&str>,
	dry: bool,
	verbose: bool,
	spinner_message: Option<&str>,
) {
	let text = if let Some(msg) = spinner_message {
		format!("{} {}", "$".green().bold(), msg)
	} else {
		format!(
			"{} {} {}",
			"$".bold().green(),
			command_name,
			&args.join(" ")
		)
	};
	let spinner = SpinnerBuilder::new()
		.spinner(&DOTS)
		.text(format!(" {}", text))
		.start();

	if !dry {
		let cmd_path_result = which::which(command_name);
		if cmd_path_result.is_err() {
			println!("{} Cannot find binary `{}`", "Fatal".red(), command_name);
			std::process::exit(1);
		}
		let cmd_path = cmd_path_result.unwrap();
		let mut command = Command::new(cmd_path);
		command.args(&args);
		command.envs(std::env::vars());
		let output = command
			.stdout(std::process::Stdio::piped())
			.output()
			.unwrap();
		spinner.text(text);
		if output.status.success() {
			spinner.done();
		} else {
			spinner.error();
			println!(
				"{} Command \"{} {}\" failed to run.",
				"Fatal".red(),
				command_name,
				&args.join(" ")
			);
			println!("{}", String::from_utf8_lossy(&output.stdout));

			std::process::exit(1);
		}
		if verbose {
			command.status().unwrap();
		}
	} else {
		spinner.text(text);
		spinner.done();
	}
}

// tests
#[cfg(test)]
mod tests {
	use crate::cli::action;
	use crate::match_cmds;
	use std::path::PathBuf;

	use crate::config::{Arguments, CommitMessageArguments, CustomTaskOptions, GlitterRc};

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
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let config = GlitterRc {
			commit_message: "$1($2): $3+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
		};

		assert_eq!(get_commit_message(&config, &args).unwrap(), "test(a): b c")
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
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let config = GlitterRc {
			commit_message: "$1($2): $3+ : $2 | $1+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
		};

		assert_eq!(
			get_commit_message(&config, &args).unwrap(),
			"test(a): b c : a | test a b c"
		)
	}

	#[test]
	fn less_than_required_args() {
		let args = Arguments {
			action: "push".to_string(),
			arguments: vec!["test".to_string(), "a".to_string()],
			rc_path: PathBuf::new(),
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let args_2 = Arguments {
			action: "push".to_string(),
			arguments: vec!["test".to_string()],
			rc_path: PathBuf::new(),
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let config = GlitterRc {
			commit_message: "$1($2): $3+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
		};

		let config_2 = GlitterRc {
			commit_message: "$1($2): $3+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
		};

		assert!(get_commit_message(&config, &args).is_err());
		assert!(get_commit_message(&config_2, &args_2).is_err());
	}

	#[test]
	fn no_commit_message_format() {
		let args = Arguments {
			action: "push".to_string(),
			arguments: vec!["test".to_string(), "a".to_string()],
			rc_path: PathBuf::new(),
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let config = GlitterRc {
			// "$1+" is the default
			commit_message: "$1+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
		};

		assert!(get_commit_message(&config, &args).is_ok())
	}

	#[test]
	fn commit_message_arguments() {
		let args = Arguments {
			action: "push".to_string(),
			arguments: vec!["feat".to_string(), "test".to_string(), "tests".to_string()],
			rc_path: PathBuf::new(),
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
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
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
		};

		assert_eq!(
			get_commit_message(&config, &args).unwrap(),
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
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let config = GlitterRc {
			commit_message: "$1($2): $3+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
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
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let config = GlitterRc {
			commit_message: "$1($2): $3+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
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
			branch: Some(String::new()),
			dry: Some(Some(false)),
			nohost: Some(Some(false)),
			raw: Some(Some(false)),
			no_verify: Some(Some(false)),
			verbose: Some(Some(false)),
		};

		let config = GlitterRc {
			commit_message: "$1($2): $3+".to_string(),
			arguments: None,
			commit_message_arguments: None,
			fetch: None,
			custom_tasks: Some(vec![CustomTaskOptions {
				name: "fmt".to_owned(),
				execute: Some(vec!["cargo fmt".to_owned()]),
			}]),
			__default: None,
			hooks: None,
			verbose: None,
		};

		assert!(match_cmds(args, config).is_err());
	}
}
