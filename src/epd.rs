use epd_waveshare::{epd7in5_v2::Epd7in5, prelude::*};

use linux_embedded_hal::{sysfs_gpio::Direction, Delay, Pin, Spidev, SysfsPin};

pub fn get_epd(
    spi: &mut Spidev,
    delay: &mut Delay,
) -> Result<
    Epd7in5<Spidev, SysfsPin, SysfsPin, SysfsPin, SysfsPin, Delay>,
    Box<dyn std::error::Error>,
> {
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

    // Setup of the needed pins is finished here
    // Now the "real" usage of the eink-waveshare-rs crate begins
    let epd = Epd7in5::new(spi, cs_pin, busy, dc, rst, delay)?;
    Ok(epd)
}
