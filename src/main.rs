use monkeylang_rs::repl::start_repl;
use std::io::Result;

fn main() -> Result<()> {
    start_repl()?;

    Ok(())
}
