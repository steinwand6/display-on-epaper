#![deny(warnings)]
use std::{
    fs::File,
    io::{BufReader, Read},
};

use embedded_graphics::prelude::ImageDrawable;
use epd_waveshare::prelude::*;
use tinybmp::Bmp;

mod calendar;
mod config;
mod epd;
mod font_setting;
mod todos;
mod utils;

use config::Config;
use font_setting::FontSetting;

fn main() -> Result<(), std::io::Error> {
    let (mut epd, mut spi, mut delay) = epd::get_epd().unwrap();
    let mut display = epd::init_display();

    // get config
    let config = Config::get_config();

    // generate image
    let mut image = image::open(config.get_template()).unwrap();

    // get font settings
    let big_font_setting = FontSetting::new(config.get_ttf(), 40.0);
    let normal_font_setting = FontSetting::new(config.get_ttf(), 30.0);

    // draw calendar
    let x = 5;
    let y = 10;
    calendar::draw_calendar(&mut image, &big_font_setting, x, y);

    // draw tasks
    let x = 0;
    let y = 60;
    todos::draw_todotasks(
        &mut image,
        config.get_task_file(),
        &normal_font_setting,
        x,
        y,
    );

    // display
    if image.save(config.get_diplay_image()).is_ok() {
        let bmp_file = File::open(config.get_diplay_image()).unwrap();
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
