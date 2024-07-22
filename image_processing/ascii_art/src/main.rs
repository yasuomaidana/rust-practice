use std::path::Path;
use image::{ImageBuffer, Luma, Rgb, RgbImage};
use crate::ascii_painter::{reduce_image_by_sampling, to_ascii_image};
use crate::error::Error;
use crate::reader_writer::{read_image_single_channel, save_img};
use rusttype::{Font, Scale, point, PositionedGlyph};

mod reader_writer;
mod error;
mod prelude;
mod color_scale;
mod kernel;
mod ascii_painter;


fn create_image_with_text(strings: &Vec<Vec<&str>>, gray_scale_color: &Vec<Vec<f64>>, filename: &str) {
    let font_data = include_bytes!("DejaVuSans.ttf") as &[u8]; // Load the font data
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    let scale = Scale::uniform(24.0); // Set the scale for the font
    let width = 18000u32; // Set your desired image width
    let height = 16000u32; // Set your desired image height

    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    for (y, row) in strings.iter().enumerate() {
        for (x, &text) in row.iter().enumerate() {
            let color_value = gray_scale_color[y][x];
            let rgb_value = (color_value * 255.0) as u8;
            let color = Rgb([rgb_value, rgb_value, rgb_value]);

            let glyphs = layout_text(&font, scale, text, x, y);

            for glyph in glyphs {
                draw_glyph(&mut image, &glyph, color);
            }
        }
    }

    image.save(filename).expect("Failed to save the image");
}

fn layout_text<'a>(font: &'a Font, scale: Scale, text: &str, x: usize, y: usize) -> Vec<PositionedGlyph<'a>> {
    let start = point(10.0 + x as f32 * 60.0, 30.0 + y as f32 * 60.0); // Adjust starting position as needed
    font.layout(text, scale, start).collect()
}

fn draw_glyph(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, glyph: &PositionedGlyph, color: Rgb<u8>) {
    if let Some(bounding_box) = glyph.pixel_bounding_box() {
        glyph.draw(|gx, gy, gv| {
            let x = gx + bounding_box.min.x as u32;
            let y = gy + bounding_box.min.y as u32;
            if x < image.width() && y < image.height() {
                let pixel = image.get_pixel_mut(x, y);
                let blend = (gv * color[0] as f32) as u8;
                *pixel = Rgb([blend, blend, blend]);
            }
        });
    }
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
