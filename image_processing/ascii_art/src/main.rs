use crate::reader_writer::read_image_single_channel;

mod reader_writer;
mod error;
mod prelude;
mod color_scale;

fn main() {
    println!("Hello, world!");
    let scaled_image =  read_image_single_channel("Sign.jpg",
                                                  &color_scale::ColorScale::new(0.2989, 0.587, 0.114));

}
