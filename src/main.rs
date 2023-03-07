#![deny(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
    prelude::{ImageDrawable, Point},
    text::{Baseline, Text, TextStyleBuilder},
    Drawable,
};

use epd_waveshare::{
    epd7in5_v2::{Display7in5, Epd7in5},
    prelude::*,
};
use image::Rgba;
use imageproc::drawing::draw_text_mut;
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, Pin, Spidev,
};
use rusttype::{Font, Scale};

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

    // Configure Digital I/O Pin to be used as Chip Select for SPI
    let cs_pin = Pin::new(26);
    cs_pin.export().expect("cs_pin export");
    while !cs_pin.is_exported() {}
    cs_pin
        .set_direction(Direction::Out)
        .expect("cs_pin Direction");
    cs_pin.set_value(1).expect("cs_pin Value set to 1");

    // Configure Busy Input Pin
    let busy = Pin::new(24);
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");
    //busy.set_value(1).expect("busy Value set to 1");

    // Configure Data/Command OutputPin
    let dc = Pin::new(25);
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    // Configure Reset OutputPin
    let rst = Pin::new(17); //pin 36 //bcm16
    rst.export().expect("rst export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");
    rst.set_value(1).expect("rst Value set to 1");

    // Configure Delay
    let mut delay = Delay {};

    // Setup of the needed pins is finished here
    // Now the "real" usage of the eink-waveshare-rs crate begins
    let mut epd = Epd7in5::new(&mut spi, cs_pin, busy, dc, rst, &mut delay)?;

    let mut display = Display7in5::default();
    display.set_rotation(DisplayRotation::Rotate270);

    // generate image
    let mut image = image::open("assets/images/tabula_rasa.bmp").unwrap();
    let font = Vec::from(include_bytes!("../assets/fonts/PlemolJPConsoleNF-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let font_size = 30.0;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };

    let text = "テスト。こんばんは世界!";
    draw_text_mut(&mut image, Rgba([0, 0, 0, 0]), 20, 50, scale, &font, text);

    let generate_file = "assets/images/task.bmp";
    if image.save(generate_file).is_ok() {
        let bmp_data = include_bytes!("../assets/images/task.bmp");
        let bmp = tinybmp::Bmp::from_slice(bmp_data).unwrap();

        display.clear_buffer(Color::Black);
        bmp.draw(&mut display).unwrap();

        epd.update_and_display_frame(&mut spi, display.buffer(), &mut delay)
            .unwrap();
    }

    Ok(())
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