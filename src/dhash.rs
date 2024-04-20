use image::{
    imageops::{grayscale, resize, FilterType::Nearest},
    DynamicImage, ImageBuffer, Luma,
};

// A representation of a difference hash
pub struct Dhash {
    hash: String,
}

impl Dhash {
    // Creates a difference hash using the given image
    pub fn new(image: DynamicImage) -> Dhash {
        let processed = Self::preprocess_image(image);
        let _ = Self::compute_difference(processed);
        Dhash {
            hash: "hello".to_string(),
        }
    }

    // Preprocesses the image by converting it to a 9x8 grayscale image
    fn preprocess_image(image: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let resize_width = 9;
        let resize_height = 8;

        let grayscale_image = grayscale(&image);

        let resized_image = resize(&grayscale_image, resize_width, resize_height, Nearest);
        resized_image
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

        println!("{}", difference.len());

        difference
    }
}
