
const FONT_DATA: &[u8] = include_bytes!("DejaVuSans.ttf") as &[u8]; // Load the font data

pub struct  GrayScaleArt {
    pub picture: Vec<Vec<u8>>,
}

pub struct ColorArt {
    pub picture: Vec<Vec<[u8; 3]>>,
}

pub trait AsciiArt {
    fn new(filename:&str) -> Self;

    fn scaled_new(filename: &str, scale: usize) -> Self;

    fn scale(&mut self, scale: usize) -> Self;
}


impl AsciiArt for GrayScaleArt{
    fn new(filename: &str) -> Self {

        todo!()
    }

    fn scaled_new(filename: &str, scale: usize) -> Self {
        todo!()
    }

    fn scale(&mut self, scale: usize) -> Self {
        todo!()
    }
}