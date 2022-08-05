use glitter::config::Arguments;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
	match Arguments::from_args_safe() {
		Ok(args) => glitter::run(args),
		Err(err) => {
			if err.use_stderr() {
				Err(err.into())
			} else {
				println!("{}", err.message);
				Ok(())
			}
		}
	}
}
// tests
#[cfg(test)]
mod tests {
	use super::main;

	#[test]
	fn runs_correctly() {
		// main will always error as it doesnt get any args
		assert!(main().is_err());
	}
}
