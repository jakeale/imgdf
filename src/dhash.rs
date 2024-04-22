use std::num::ParseIntError;

use image::{
    imageops::{grayscale, resize, FilterType::Triangle},
    DynamicImage, ImageBuffer, Luma,
};

/// A representation of a difference hash
#[derive(Debug)]
pub struct Dhash {
    hash: String,
}

impl Dhash {
    /// Calculates the hamming distance between two `Dhash` instances
    ///
    /// Compares their hashes by counting the number of bits that are different
    ///
    /// # Return Values
    /// - `value > 10` means its likely a different image
    /// - `1 < value < 10` means its a potential variation
    /// - a value of `0` means its likely a similar picture
    pub fn hamming_distance(&self, other: Dhash) -> Result<usize, ParseIntError> {
        let difference = self.hash_to_u64()? ^ other.hash_to_u64()?;

        let binary = format!("{:b}", difference);
        let distance = binary.chars().filter(|&c| c == ('1')).count();

        Ok(distance)
    }

    fn hash_to_u64(&self) -> Result<u64, ParseIntError> {
        Ok(u64::from_str_radix(&self.hash, 16)?)
    }
}

/// Creates a difference hash using the given image
pub fn calculate_dhash(image: &DynamicImage) -> Result<Dhash, ParseIntError> {
    let processed = preprocess_image(image);
    let difference = compute_difference(processed);

    let hash = encode(difference)?;

    Ok(Dhash { hash })
}

/// Encodes the difference by converting it from binary array -> `String`
fn encode(difference: Vec<bool>) -> Result<String, ParseIntError> {
    let binary: String = difference
        .iter()
        .map(|&b| if b { '1' } else { '0' })
        .collect();

    // convert base-2 string to u64
    let num = u64::from_str_radix(&binary, 2)?;

    // convert u64 to hexadecimal
    Ok(format!("{:X}", num))
}

/// Computes the difference of adjacent pixels by checking if the right pixel is brighter or not
///
/// An 9x8 image results in 8 rows with 8 differences
fn compute_difference(image: ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<bool> {
    let mut difference: Vec<bool> = Vec::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        if let Some(right_pixel) = image.get_pixel_checked(x + 1, y) {
            difference.push(pixel[0] > right_pixel[0])
        }
    }

    difference
}

/// Preprocesses the image by converting it to a 9x8 grayscale image
fn preprocess_image(image: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let resize_width = 9;
    let resize_height = 8;

    let resized_image = resize(image, resize_width, resize_height, Triangle);

    grayscale(&resized_image)
}
