use crate::config::GlitterRc;
use anyhow::Context;
use std::fs::File;
use std::path::Path;
// parse the config file
pub fn parse(path: &Path) -> anyhow::Result<GlitterRc> {
	let does_exist = Path::new(path).exists();
	if !does_exist {
		Ok(GlitterRc {
			fetch: None,
			commit_message: "$1+".to_owned(),
			arguments: None,
			custom_tasks: None,
			commit_message_arguments: None,
			__default: Some(true),
			hooks: None,
			verbose: None,
		})
	} else {
		let file = File::open(path)?;
		match serde_json::from_reader(file) {
			Ok(json) => Ok(json),
			Err(err) => Err(anyhow::Error::new(err)).with_context(|| "error parsing glitterrc"),
		}
	}
}
