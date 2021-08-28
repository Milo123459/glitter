use glitter::config::Arguments;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
	glitter::run(Arguments::from_args_safe()?)?;

	Ok(())
}
