use std::error::Error;

use clap::Parser;

use piet_programming_language::args::Args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    args.validate()?;
    piet_programming_language::run(&args)
}
