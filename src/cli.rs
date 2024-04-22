use clap::{Arg, Command, Parser};

#[derive(Parser)]
#[command(about, arg_required_else_help = true)]
#[group(required = true, multiple = false)]
pub struct Cli {
    /// Path to two different images
    #[arg(num_args = 2)]
    image: Option<Vec<String>>,

    /// Path to a directory
    #[arg(last = true)]
    directory: Option<String>,
}
fn cli() -> Command {
    Command::new("imgdf")
        .about("image similarity utility")
        .arg_required_else_help(true)
        .max_term_width(80)
        .arg(
            Arg::new("image")
                .help(
                    "Two paths to two images. \
                    This will output the hamming distance between two images. \
                    The lower this value, the more likely they are the same.",
                )
                .num_args(2)
                .conflicts_with("directory"),
        )
        .arg(
            Arg::new("directory")
                .help("path to a directory")
                .num_args(1)
                .last(true),
        )
}
