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

    /// Treats unknown colors as white instead of error
    #[arg(long)]
    pub fall_back_to_white: bool,

    /// Treats unknown colors as black instead of error
    #[arg(long)]
    pub fall_back_to_black: bool,

    /// Terminates the program after this number of iterations
    #[arg(long)]
    pub max_iter: Option<usize>,

    /// Enables debug output (path trace etc.)
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        if self.fall_back_to_white && self.fall_back_to_black {
            return Err(
                "at most one of `fall_back_to_white` and `fall_back_to_black` can be set"
                    .to_string(),
            );
        }

        Ok(())
    }
}
