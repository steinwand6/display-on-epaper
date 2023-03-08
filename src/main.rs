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

use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    Delay, Spidev,
};
use rusttype::{Font, Scale};
use tinybmp::Bmp;

mod epd;

struct FontSetting<'a> {
    font: Font<'a>,
    scale: (f32, f32),
}

fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    // SPI settings are from eink-waveshare-rs documenation
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("spi configuration");

    // Configure Delay
    let mut delay = Delay {};

    let mut epd = epd::get_epd(&mut spi, &mut delay).unwrap();

    let mut display = Display7in5::default();
    display.clear_buffer(Color::Black);
    display.set_rotation(DisplayRotation::Rotate270);

    // generate image
    let mut image = image::open("assets/images/tabula_rasa.bmp").unwrap();
    let font = Vec::from(include_bytes!("../assets/fonts/PlemolJPConsoleNF-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let font_setting = FontSetting {
        font,
        scale: (30.0, 30.0),
    };

    // get tasks
    let mut tasks = Vec::new();
    let task_list = File::open("task.org").unwrap();
    let reader = BufReader::new(task_list);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(" TODO ") {
            let line = line.replace(" TODO ", "").replace("*", " ");
            tasks.push(line);
        }
    }

    let x = 5;
    let mut y = 10;
    for text in tasks {
        draw_text_on_image(&mut image, x, y, text.as_str(), &font_setting);
        y += 35;
    }

    let bmp_file = "assets/images/task.bmp";
    if image.save(bmp_file).is_ok() {
        let bmp_file = File::open(bmp_file).unwrap();
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

fn _draw_text(display: &mut Display7in5, text: &str, x: i32, y: i32) {
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
