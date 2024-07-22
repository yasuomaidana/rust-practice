use crate::ascii_painter::{reduce_image_by_sampling, to_ascii_image};
use crate::error::Error;
use crate::reader_writer::{read_image_single_channel, save_img};

mod reader_writer;
mod error;
mod prelude;
mod color_scale;
mod kernel;
mod ascii_painter;

fn main() -> Result<(), Error> {
    let (width, height, color_scaled_image) =  read_image_single_channel("Sign.jpg",
                                                                         &color_scale::ColorScale::new(0.2989, 0.587, 0.114))?;

    let reduced_image = reduce_image_by_sampling(&color_scaled_image, 10);
    let (reduced_width, reduced_height) = (reduced_image[0].len(), reduced_image.len());
    save_img(reduced_width as u32, reduced_height as u32, "reduced.jpg", &reduced_image)?;

    let mut ascii_image = to_ascii_image(&reduced_image);

    for row in ascii_image {
        for pixel in row {
            print!("{}", pixel);
        }
        println!();
    }
    Ok(())
}
