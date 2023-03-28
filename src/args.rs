use clap::Parser;

/// Interpreter for Piet Programming Language
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub image_file: String,

    /// Enables debug output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn new(image_file: &str, verbose: bool) -> Self {
        Self {
            image_file: image_file.to_owned(),
            verbose,
        }
    }
}
