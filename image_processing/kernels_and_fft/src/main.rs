mod prelude;
use crate::prelude::*;

mod kernel;

mod error;

use image::{GenericImageView, GrayImage};


struct ColorScale {
    r_scale: f64,
    g_scale: f64,
    b_scale: f64,
}

impl ColorScale {
    /// Creates a new `ColorScale` ensuring the sum of its elements equals 1.
    /// If the sum is not 1, it normalizes the components so that their sum will be 1.
    ///
    /// # Arguments
    ///
    /// * `r_scale` - The red scale component.
    /// * `g_scale` - The green scale component.
    /// * `b_scale` - The blue scale component.
    ///
    /// # Returns
    ///
    /// A new instance of `ColorScale`.
    pub fn new(mut r_scale: f64, mut g_scale: f64, mut b_scale: f64) -> Self {

        let sum = r_scale + g_scale + b_scale;
        if sum != 0.0 {
            r_scale /= sum;
            g_scale /= sum;
            b_scale /= sum;
        }
        ColorScale { r_scale, g_scale, b_scale }
    }
}

/// Reads an image from a file and converts it to a grayscale image based on a given color scale.
///
/// This function opens an image file, reads its pixels, and converts each pixel to a grayscale
/// value. The grayscale value is calculated using a weighted sum of the RGB components, where
/// the weights are specified by the `rgb_gray_scale` argument. The result is a 2D vector of
/// floating-point numbers representing the grayscale image, where each element corresponds to
/// the grayscale value of a pixel.
///
/// # Arguments
///
/// * `image_name` - A string slice that holds the name (and path) of the image file to be read.
/// * `rgb_gray_scale` - A reference to a `ColorScale` instance that specifies the weights for
///   the RGB components in the grayscale conversion.
///
/// # Returns
///
/// A `Result` containing a tuple with the image's width and height (in pixels) and the 2D vector
/// of grayscale values. Returns an error if the image cannot be opened or read.
///
/// # Examples
///
/// ```
/// let color_scale = ColorScale::new(0.2989, 0.587, 0.114);
/// let image_result = read_image("path/to/image.jpg", &color_scale);
/// if let Ok((width, height, gray_img)) = image_result {
///     println!("Image dimensions: {}x{}", width, height);
///     // Use `gray_img` for further processing...
/// }
/// ```
fn read_image(image_name: &str, rgb_gray_scale: &ColorScale) -> Result<((u32, u32, Vec<Vec<f64>>))> {
    // read the image
    let img = image::open(image_name)?;

    let (width, height) = img.dimensions();

    // create an array to hold the image
    let mut gray_img = vec![vec![0.; width as usize]; height as usize];
    // loop through and calculate the pixel values as 0-1 grey float values
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.0;
            let gray =
                (rgb_gray_scale.r_scale * rgb[0] as f64 +
                    rgb_gray_scale.g_scale * rgb[1] as f64 +
                    rgb_gray_scale.b_scale * rgb[2] as f64) / 255.;
            gray_img[y as usize][x as usize] = gray;
        }
    }

    return Ok((width, height, gray_img));
}



fn main() {
    println!("Hello, world!");
    let color_scale = ColorScale::new(0.2989, 0.587, 0.114);
    let image = read_image("Cat.jpg", &color_scale);

}
