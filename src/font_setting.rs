use rusttype::Font;

use crate::utils;

pub struct FontSetting<'a> {
    font: Font<'a>,
    scale: (f32, f32),
}

impl FontSetting<'_> {
    pub fn new(ttf_file_path: &str, scale: f32) -> Self {
        let font_date = utils::get_bytes_from_filepath(ttf_file_path);

        Self {
            font: Font::try_from_vec(Vec::from(font_date.as_slice())).unwrap(),
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
