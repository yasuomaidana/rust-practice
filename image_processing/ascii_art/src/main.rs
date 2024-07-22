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



fn calculate_image_dimensions(strings: &Vec<Vec<&str>>, font: &Font, scale: Scale) -> (u32, u32) {
    let mut max_width = 0.0f32;
    let mut total_height = 0.0f32;

    for row in strings {
        let mut row_width = 0.0f32;
        let mut row_height = 0.0f32;

        for &text in row {
            let (text_width, text_height) = calculate_text_dimensions(text, font, scale);
            row_width += text_width;
            row_height = row_height.max(text_height);
        }

        max_width = max_width.max(row_width);
        total_height += row_height;
    }

    (max_width as u32, total_height as u32)
}

fn calculate_text_dimensions(text: &str, font: &Font, scale: Scale) -> (f32, f32) {
    let v_metrics = font.v_metrics(scale);
    let height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap).ceil();
    let width = text.chars().map(|c| {
        let glyph = font.glyph(c).scaled(scale);
        glyph.h_metrics().advance_width
    }).sum::<f32>();

    (width, height)
}

fn create_image_with_text(strings: &Vec<Vec<&str>>, gray_scale_color: &Vec<Vec<f64>>, filename: &str) {
    let font_data = include_bytes!("DejaVuSans.ttf") as &[u8]; // Load the font data
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
    let font_size = 24.0;

    let scale = Scale::uniform(font_size); // Set the scale for the font

    let (height,width ) = calculate_image_dimensions(&strings, &font, scale);

    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(
        width, height);

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
    let start = point(scale.x + x as f32 * scale.x, scale.y + y as f32 * scale.y); // Adjust starting position as needed
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

    let name = "Sign";
    let file_name = format!("{}.jpg", name);
    let reduced_name = format!("{}_reduced.jpg", name);
    let ascii_name = format!("{}_ascii.jpg", name);

    let (_, _, color_scaled_image) =  read_image_single_channel(file_name.as_str(),
                                                                         &color_scale::ColorScale::new(0.2989, 0.587, 0.114))?;

    let reduced_image = reduce_image_by_sampling(&color_scaled_image, 2);
    let (reduced_width, reduced_height) = (reduced_image[0].len(), reduced_image.len());
    save_img(reduced_width as u32, reduced_height as u32, reduced_name.as_str(), &reduced_image)?;

    let ascii_image = to_ascii_image(&reduced_image);

    create_image_with_text(&ascii_image, &reduced_image, ascii_name.as_str());

    for row in ascii_image {
        for pixel in row {
            print!("{}", pixel);
        }
        println!();
    }
    Ok(())
}
