use crate::config::GlitterRc;
use anyhow::Context;
use std::fs::File;
use std::path::PathBuf;

pub fn parse(path: &PathBuf) -> anyhow::Result<GlitterRc> {
    let file = File::open(path.as_path())
        .with_context(|| format!("Could not read file `{}`", path.as_path().to_str().unwrap()))?;

    match serde_json::from_reader(file) {
        Ok(json) => Ok(json),
        Err(err) => Err(anyhow::Error::new(err)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::GlitterRc;

    use super::parse;

    #[test]
    fn parse_correctly() {
        assert_eq!(
            parse(&PathBuf::from(".glitterrc")).unwrap(),
            GlitterRc {
                commit_message: "$1: $2+".to_string(),
                arguments: None,
                commit_message_arguments: None
            }
        )
    }

    #[test]
    fn non_existant_file() {
        assert_eq!(parse(&PathBuf::from(".glitter")).is_err(), true)
    }
}
