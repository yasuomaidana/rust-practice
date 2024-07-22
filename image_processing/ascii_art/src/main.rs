use std::path::Path;
use image::{ImageBuffer, Luma, Rgb, RgbImage};
use crate::ascii_painter::{reduce_image_by_sampling, to_ascii_image};
use crate::error::Error;
use crate::reader_writer::{read_image_single_channel, save_img};

mod reader_writer;
mod error;
mod prelude;
mod color_scale;
mod kernel;
mod ascii_painter;

struct Font(Vec<u8>);





fn create_image_with_text(strings: &Vec<Vec<&str>>, gray_scale_color: &Vec<Vec<f64>>, filename: &str) {
    let font_data = include_bytes!("DejaVuSans.ttf");
    let font = Font(font_data.to_vec());

    // Implementation for using the font, grayscale values, and filename to create an image
    // This part of the code is not provided but would go here.
}

fn main() -> Result<(), Error> {
    let (width, height, color_scaled_image) =  read_image_single_channel("Sign.jpg",
                                                                         &color_scale::ColorScale::new(0.2989, 0.587, 0.114))?;

    let reduced_image = reduce_image_by_sampling(&color_scaled_image, 5);
    let (reduced_width, reduced_height) = (reduced_image[0].len(), reduced_image.len());
    save_img(reduced_width as u32, reduced_height as u32, "reduced.jpg", &reduced_image)?;

    let mut ascii_image = to_ascii_image(&reduced_image);

    create_image_with_text(&ascii_image, &reduced_image, "ascii_art.jpg");
    // create_grayscale_image(&ascii_image, &reduced_image, "ascii_art.jpg");

    for row in ascii_image {
        for pixel in row {
            print!("{}", pixel);
        }
        println!();
    }
    Ok(())
}
