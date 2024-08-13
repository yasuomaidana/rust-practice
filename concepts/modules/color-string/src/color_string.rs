use crate::color_text::{formatter, Color};

pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized:String
}

impl ColorString {
    pub fn new(color: Option<Color>, string: &str) -> ColorString {
        let color = color.unwrap_or(Color::Default);
        let colorized = formatter(&color, string);
        ColorString {
            color,
            string: string.to_string(),
            colorized
        }
    }

    pub fn paint(&mut self) {
        self.colorized = formatter(&self.color, &self.string);
    }

    pub fn reset(&mut self) {
        self.colorized = formatter(&Color::Default, &self.string);
    }

    pub fn change_color(&mut self, color: Color) {
        self.color = color;
        self.paint();
    }
}