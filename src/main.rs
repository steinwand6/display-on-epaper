#![deny(warnings)]
use embedded_graphics::prelude::ImageDrawable;
use epd_waveshare::prelude::*;
use tinybmp::Bmp;

mod calendar;
mod config;
mod epd;
mod font_setting;
mod quotes;
mod todos;
mod utils;

use config::Config;
use font_setting::FontSetting;

fn main() -> Result<(), std::io::Error> {
    let (mut epd, mut spi, mut delay) = epd::get_epd().unwrap();
    let mut display = epd::init_display();

    // get config
    let config = Config::get_config();

    // get template image
    let mut image = image::open(config.get_template()).unwrap();

    // get font settings
    let big_font_setting = FontSetting::new(config.get_ttf(), 40.0);
    let normal_font_setting = FontSetting::new(config.get_ttf(), 30.0);
    let quote_font_setting = FontSetting::new(config.get_ttf_for_quote(), 28.0);

    // draw calendar
    let x = 5;
    let y = 10;
    calendar::draw_calendar(&mut image, &big_font_setting, x, y);

    // draw tasks
    let x = 0;
    let y = 70;
    todos::draw_todotasks(
        &mut image,
        config.get_task_file(),
        &normal_font_setting,
        x,
        y,
    );

    // draw quote
    let x = 10;
    let y = 680;
    quotes::draw_quote(
        &mut image,
        &config.get_quotes_file(),
        &quote_font_setting,
        x,
        y,
    );

    // display
    if image.save(config.get_diplay_image()).is_ok() {
        let image_data = utils::get_bytes_from_filepath(config.get_diplay_image());
        let bmp = Bmp::from_slice(&image_data.as_slice()).unwrap();
        bmp.draw(&mut display).unwrap();
        epd.update_and_display_frame(&mut spi, display.buffer(), &mut delay)
            .unwrap();
    }

    Ok(())
}
