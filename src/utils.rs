use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Baseline, Text, TextStyleBuilder},
    Drawable,
};
use epd_waveshare::epd7in5_v2::Display7in5;
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::Scale;

use crate::FontSetting;

pub fn draw_texts_on_image(
    image: &mut DynamicImage,
    x: i32,
    mut y: i32,
    texts: Vec<String>,
    font: &FontSetting,
) {
    for text in texts {
        draw_text_on_image(image, x, y, text.as_str(), &font);
        y += font.scale.1 as i32;
    }
}

pub fn draw_text_on_image(
    image: &mut DynamicImage,
    x: i32,
    y: i32,
    text: &str,
    font_setting: &FontSetting,
) {
    let scale = Scale {
        x: font_setting.scale.0,
        y: font_setting.scale.1,
    };
    draw_text_mut(
        image,
        Rgba([0, 0, 0, 0]),
        x,
        y,
        scale,
        &font_setting.font,
        text,
    );
}

#[allow(unused)]
pub fn draw_text(display: &mut Display7in5, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::jis_x0201::FONT_10X20)
        .background_color(BinaryColor::On)
        .text_color(BinaryColor::Off)
        .build();
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    Text::with_text_style(text, Point::new(x, y), style, text_style)
        .draw(display)
        .unwrap();
}
