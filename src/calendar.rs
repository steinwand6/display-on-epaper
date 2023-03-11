use chrono::{Local, Timelike};
use image::DynamicImage;

use crate::utils;
use crate::FontSetting;

pub fn draw_calendar(image: &mut DynamicImage, font_setting: &FontSetting, x: i32, y: i32) {
    let now = Local::now();
    let (is_pm, hour) = now.hour12();
    let calendar = now.format("%Y/%m/%d").to_string() + "  " + hour.to_string().as_str();
    let calendar = calendar + if is_pm { "PM" } else { "AM" };
    utils::draw_text_on_image(image, x, y, calendar.to_string().as_str(), font_setting);
}
