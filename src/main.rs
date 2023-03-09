#![deny(warnings)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
    prelude::{ImageDrawable, Point},
    text::{Baseline, Text, TextStyleBuilder},
    Drawable,
};

use epd_waveshare::{epd7in5_v2::Display7in5, prelude::*};
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use serde::{Deserialize, Serialize};

use rusttype::{Font, Scale};
use tinybmp::Bmp;

mod epd;

struct FontSetting<'a> {
    font: Font<'a>,
    scale: (f32, f32),
}

#[derive(Serialize, Deserialize)]
struct Config {
    template_path: String,
    display_image_path: String,
    ttf_path: String,
    task_file_path: String,
}

fn main() -> Result<(), std::io::Error> {
    let (mut epd, mut spi, mut delay) = epd::get_epd().unwrap();
    let mut display = init_display();

    let config = include_str!("../config.toml");
    let config: Config = toml::from_str(config).unwrap();

    // generate image
    let mut image = image::open(&config.template_path).unwrap();
    let f = File::open(&config.ttf_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();
    let font = Vec::from(buffer.as_slice());
    let font = Font::try_from_vec(font).unwrap();
    let font_setting = FontSetting {
        font,
        scale: (30.0, 30.0),
    };

    // get tasks
    let mut tasks = Vec::new();
    let task_list = File::open(&config.task_file_path).unwrap();
    let reader = BufReader::new(task_list);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(" TODO ") {
            let line = line.replace(" TODO ", "").replace("*", " ");
            tasks.push(line);
        }
    }

    let x = 5;
    let y = 10;
    draw_texts_on_image(&mut image, x, y, tasks, &font_setting);

    if image.save(&config.display_image_path).is_ok() {
        let bmp_file = File::open(&config.display_image_path).unwrap();
        let mut reader = BufReader::new(bmp_file);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer).unwrap();
        let bmp = Bmp::from_slice(buffer.as_slice()).unwrap();

        bmp.draw(&mut display).unwrap();

        epd.update_and_display_frame(&mut spi, display.buffer(), &mut delay)
            .unwrap();
    }

    Ok(())
}

fn draw_texts_on_image(
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

fn init_display() -> Display7in5 {
    let mut display = Display7in5::default();
    display.clear_buffer(Color::Black);
    display.set_rotation(DisplayRotation::Rotate270);
    display
}

fn draw_text_on_image(
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
fn draw_text(display: &mut Display7in5, text: &str, x: i32, y: i32) {
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
