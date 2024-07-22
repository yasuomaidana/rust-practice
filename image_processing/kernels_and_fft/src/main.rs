mod prelude;
use crate::prelude::*;

mod kernel;

mod error;
mod padding;
mod convolve;

use image::{GenericImageView, GrayImage};
use crate::kernel::{Kernel, KernelConstructors};
use crate::padding::reflection_pad;

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
fn read_image(image_name: &str, rgb_gray_scale: &ColorScale) -> Result<(u32, u32, Vec<Vec<f64>>)> {
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


/// Saves a grayscale image to a file.
///
/// This function takes a 2D vector representing a grayscale image, where each element is a
/// floating-point number between 0.0 and 1.0 indicating the intensity of a pixel, and saves
/// this image to a file. The function converts these floating-point values to 8-bit integers
/// (0-255 scale) and constructs a `GrayImage` before saving it to the specified path.
///
/// # Arguments
///
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
/// * `image_name` - The path and name of the file where the image will be saved.
/// * `img` - A reference to a 2D vector of `f64` representing the grayscale values of the image.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the save operation. On success, it returns `Ok(())`.
///
/// # Examples
///
/// ```
/// let img = vec![vec![0.5; 100]; 100]; // A 100x100 image with all pixels at 50% intensity
/// save_img(100, 100, "path/to/save_image.png", &img).expect("Failed to save image");
/// ```
fn save_img(width: u32, height: u32, image_name: &str, img: &Vec<Vec<f64>>) -> Result<()> {
    // convert to grey image
    let mut ouput = GrayImage::new(width, height);
    for (x, y, pixel) in ouput.enumerate_pixels_mut() {
        let gray_value = (img[y as usize][x as usize] * 255.0) as u8;
        *pixel = image::Luma([gray_value]);
    }

    // save the image
    ouput.save(image_name)?;

    return Ok(());
}

fn main() -> std::result::Result<(), Error> {
    println!("Cat to grayscale");
    let color_scale = ColorScale::new(0.2989, 0.587, 0.114);
    let (width, height, img) = read_image("Cat.jpg", &color_scale)?;
    save_img(width, height, "cat_gray.jpg", &img)?;
    let padded_img = reflection_pad(&img, 3);
    let new_height = padded_img.len() as u32;
    let new_width = padded_img[0].len() as u32;
    save_img(new_width, new_height, "cat_gray_padded.jpg", &padded_img)?;

    let kernel = Kernel::blur(2.0);
    let convolved_img = convolve::conv_2d(&img, &kernel);
    save_img(width, height, "cat_gray_blurred.jpg", &convolved_img)?;
    return Ok(());

}
