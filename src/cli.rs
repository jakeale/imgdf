use std::path::PathBuf;

use anyhow::anyhow;
use clap::{Arg, Command, ValueHint};
use image::ImageFormat;

fn validate_images(images: &Vec<PathBuf>) -> Result<(), anyhow::Error> {
    let is_valid = |path: &PathBuf| {
        path.exists()
            && path.is_file()
            && ImageFormat::from_path(path).is_ok_and(|format| format.can_read())
    };

    if let Some(invalid_image) = images.iter().find(|&image| !is_valid(image)) {
        return Err(anyhow!("Invalid path or image format: {:?}", invalid_image));
    }

    Ok(())
}

pub fn parse_args() -> Result<Vec<PathBuf>, anyhow::Error> {
    let cli = cli();
    let matches = cli.get_matches();

    let images: Vec<PathBuf> = matches
        .get_many::<PathBuf>("image")
        .unwrap_or_default()
        .cloned()
        .collect();

    validate_images(&images)?;

    Ok(images)
}

fn cli() -> Command {
    Command::new("imgsf")
        .about("image similarity finder")
        .arg_required_else_help(true)
        .max_term_width(20)
        .arg(
            Arg::new("image")
                .help(
                    "Two paths to two images. \
                    This will output the hamming distance between two images. \
                    The lower this value, the more likely they are the same.",
                )
                .num_args(2)
                .value_name("IMAGE_PATH")
                .value_hint(ValueHint::FilePath)
                .value_parser(clap::value_parser!(PathBuf)),
        )
    // .arg(
    //     Arg::new("directory")
    //         .value_name("DIRECTORY_PATH")
    //         .long("recursive")
    //         .short('r')
    //         .help("path to a directory")
    //         .num_args(1)
    //         .conflicts_with("image"),
    // )
}
