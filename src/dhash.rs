use std::num::ParseIntError;

use image::{
    imageops::{grayscale, resize, FilterType::Nearest},
    DynamicImage, ImageBuffer, Luma,
};

// A representation of a difference hash
pub struct Dhash {
    hash: u64,
}

// Creates a difference hash using the given image
pub fn create_dhash(image: DynamicImage) -> Result<Dhash, ParseIntError> {
    let processed = preprocess_image(image);
    let difference = compute_difference(processed);

    let hash = calculate_hash(difference)?;

    Ok(Dhash { hash })
}

fn calculate_hash(difference: Vec<bool>) -> Result<u64, ParseIntError> {
    let binary: String = difference
        .iter()
        .map(|&b| if b { '1' } else { '0' })
        .collect();

    let hex = u64::from_str_radix(&binary, 2);
    hex
}

// Computes the difference of adjacent pixels by checking if the right pixel is brighter or not
// An 9x8 image results in 8 rows with 8 differences, which results in a 64 bit array
fn compute_difference(image: ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<bool> {
    let mut difference: Vec<bool> = Vec::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        let right_pixel = image.get_pixel_checked(x + 1, y);

        if right_pixel.is_some() {
            difference.push(pixel[0] > right_pixel.unwrap()[0])
        }
    }

    difference
}

// Preprocesses the image by converting it to a 9x8 grayscale image
fn preprocess_image(image: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let resize_width = 9;
    let resize_height = 8;

    let grayscale_image = grayscale(&image);

    let resized_image = resize(&grayscale_image, resize_width, resize_height, Nearest);
    resized_image
}
