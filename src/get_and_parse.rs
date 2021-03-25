use crate::config::GlitterRc;
use anyhow::Context;
use std::fs::File;
use std::path::{Path, PathBuf};
// parse the config file
pub fn parse(path: &PathBuf) -> anyhow::Result<GlitterRc> {
	let does_exist = Path::new(path.as_path()).exists();
	if !does_exist {
		Ok(GlitterRc {
			fetch: None,
			commit_message: "$1+".to_owned(),
			arguments: None,
			custom_tasks: None,
			commit_message_arguments: None,
			__default: Some(true),
		})
	} else {
		let file = File::open(path.as_path())?;
		match serde_json::from_reader(file) {
			Ok(json) => Ok(json),
			Err(err) => Err(anyhow::Error::new(err)).with_context(|| "error parsing glitterrc"),
		}
	}
}
// tests
#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use crate::config::{CommitMessageArguments, CustomTaskOptions, GlitterRc};

	use super::parse;

	#[test]
	fn parse_correctly() {
		assert_eq!(
			parse(&PathBuf::from(".glitterrc")).unwrap(),
			GlitterRc {
				commit_message: "$1: $2: $3+".to_string(),
				arguments: None,
				commit_message_arguments: Some(vec![CommitMessageArguments {
					argument: 1,
					case: Some("lower".to_string()),
					type_enums: Some(vec![
						"fix".to_string(),
						"feat".to_string(),
						"chore".to_string(),
						"refactor".to_string(),
						"docs".to_string(),
						"void".to_string(),
						"deps".to_string()
					])
				}]),
				fetch: None,
				custom_tasks: Some(vec![CustomTaskOptions {
					name: "fmt".to_owned(),
					execute: Some(vec!["cargo fmt".to_owned()])
				}]),
				__default: None
			}
		)
	}

	#[test]
	fn non_existant_file() {
		assert_eq!(
			parse(&PathBuf::from(".glitter"))
				.unwrap()
				.__default
				.is_some(),
			true
		)
	}

	#[test]
	fn broken_glitterrc() {
		assert_eq!(parse(&PathBuf::from(".glitterrc.broken")).is_err(), true)
	}
}
