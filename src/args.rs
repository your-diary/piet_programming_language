use clap::Parser;

/// Interpreter for Piet Programming Language
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub image_file: String,

    /// Specifies the codel size (default: auto detect)
    #[arg(short, long)]
    pub codel_size: Option<usize>,

    /// Enables debug output (path trace etc.)
    #[arg(short, long)]
    pub verbose: bool,
}
