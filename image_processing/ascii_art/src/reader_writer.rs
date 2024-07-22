use crate::prelude::*;
use crate::color_scale::ColorScale;


use image::{GenericImageView, GrayImage};

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
pub(crate) fn read_image_single_channel(image_name: &str, rgb_gray_scale: &ColorScale) -> Result<(u32, u32, Vec<Vec<f64>>)> {
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

/// Reads an image from a file and retrieves the RGB channel values without scaling.
///
/// This function opens an image file, reads its pixels, and extracts the RGB channel values
/// directly without applying any scaling or conversion. The result is a 3D vector where each
/// element represents the RGB values of a pixel in the format Vec<Vec<[f64; 3]>>, with the outer
/// vectors representing the rows and columns of the image, and the inner array representing the
/// RGB values of each pixel.
///
/// # Arguments
///
/// * `image_name` - A string slice that holds the name (and path) of the image file to be read.
///
/// # Returns
///
/// A `Result` containing a tuple with the image's width and height (in pixels) and the 3D vector
/// of RGB values. Returns an error if the image cannot be opened or read.
// pub(crate) fn read_image_rgb(image_name: &str) -> Result<(u32, u32, Vec<Vec<[f64; 3]>>)> {
//     let img = image::open(image_name)?;
//
//     let (width, height) = img.dimensions();
//
//     let mut rgb_img = vec![vec![[0.0; 3]; width as usize]; height as usize];
//
//     for y in 0..height {
//         for x in 0..width {
//             let pixel = img.get_pixel(x, y);
//             let rgb = pixel.0;
//             rgb_img[y as usize][x as usize] = [
//                 rgb[0] as f64,
//                 rgb[1] as f64,
//                 rgb[2] as f64,
//             ];
//         }
//     }
//
//     Ok((width, height, rgb_img))
// }

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
pub(crate) fn save_img(width: u32, height: u32, image_name: &str, img: &Vec<Vec<f64>>) -> Result<()> {
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

// pub fn get_image_size(image_array: &Vec<Vec<f64>>) -> (u32, u32) {
//     let height = image_array.len() as u32;
//     let width = if !image_array.is_empty() { image_array[0].len() as u32 } else { 0 };
//
//     (width, height)
// }
