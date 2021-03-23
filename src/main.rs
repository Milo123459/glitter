use glitter::config::Arguments;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    glitter::run(Arguments::from_args_safe()?)?;

    Ok(())
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
