#![deny(warnings)]
use std::{
    fs::File,
    io::{BufReader, Read},
};

use embedded_graphics::prelude::ImageDrawable;
use epd_waveshare::{epd7in5_v2::Display7in5, prelude::*};
use rusttype::Font;
use serde::{Deserialize, Serialize};
use tinybmp::Bmp;

mod calendar;
mod epd;
mod todos;
mod utils;

pub struct FontSetting<'a> {
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

    // get config
    let config = include_str!("../config.toml");
    let config: Config = toml::from_str(config).unwrap();

    // generate image
    let mut image = image::open(&config.template_path).unwrap();
    let f = File::open(&config.ttf_path).unwrap();

    // get font settings
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let big_font_setting = FontSetting {
        font: Font::try_from_vec(Vec::from(buffer.as_slice())).unwrap(),
        scale: (40.0, 40.0),
    };
    let normal_font_setting = FontSetting {
        font: Font::try_from_vec(Vec::from(buffer.as_slice())).unwrap(),
        scale: (30.0, 30.0),
    };

    // draw calendar
    let x = 5;
    let y = 10;
    calendar::draw_calendar(&mut image, &big_font_setting, x, y);

    // draw tasks
    let x = 0;
    let y = 60;
    todos::draw_todotasks(
        &mut image,
        &config.task_file_path,
        &normal_font_setting,
        x,
        y,
    );

    // display
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

fn init_display() -> Display7in5 {
    let mut display = Display7in5::default();
    display.clear_buffer(Color::Black);
    display.set_rotation(DisplayRotation::Rotate270);
    display
}
