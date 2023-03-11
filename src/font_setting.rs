use std::fs::File;
use std::io::{BufReader, Read};

use rusttype::Font;

pub struct FontSetting<'a> {
    font: Font<'a>,
    scale: (f32, f32),
}

impl FontSetting<'_> {
    pub fn new(ttf_file_path: &str, scale: f32) -> Self {
        let f = File::open(ttf_file_path).unwrap();

        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        Self {
            font: Font::try_from_vec(Vec::from(buffer.as_slice())).unwrap(),
            scale: (scale, scale),
        }
    }

    pub fn get_font(&self) -> &Font<'_> {
        &self.font
    }

    pub fn get_scale(&self) -> (f32, f32) {
        self.scale
    }
}
