use image::DynamicImage;
use rand::{seq::SliceRandom, thread_rng};

use serde::{Deserialize, Serialize};

use crate::font_setting::FontSetting;
use crate::utils::draw_texts_on_image;

#[derive(Serialize, Deserialize)]
pub struct Quote {
    quote: String,
    speaker: String,
}

fn get_quotes(file_path: &str) -> Vec<Quote> {
    let mut quotes_csv = csv::Reader::from_path(file_path).unwrap();
    let mut quotes: Vec<Quote> = Vec::new();
    for quote in quotes_csv.deserialize() {
        let quote = quote.unwrap();
        quotes.push(quote);
    }
    quotes
}

fn get_quote(file_path: &str) -> Vec<String> {
    let quotes = get_quotes(file_path);
    let quote = quotes.choose(&mut thread_rng()).unwrap();
    quote.quote.split("\\n").map(|q| q.into()).collect()
}

pub fn draw_quote(
    image: &mut DynamicImage,
    quotes_file_path: &str,
    font_setting: &FontSetting,
    x: i32,
    y: i32,
) {
    let quote = get_quote(quotes_file_path);
    draw_texts_on_image(image, x, y, quote, &font_setting);
}
