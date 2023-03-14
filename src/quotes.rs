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

fn get_quote(file_path: &str) -> String {
    let quotes = get_quotes(file_path);
    let quote = quotes.choose(&mut thread_rng()).unwrap();
    quote.quote.to_string()
}

pub fn draw_quote(
    image: &mut DynamicImage,
    quotes_file_path: &str,
    font_setting: &FontSetting,
    x: i32,
    y: i32,
) {
    let quote = get_quote(quotes_file_path);
    let multibyte_weight = if !quote.chars().all(|c| c.is_ascii()) {
        2
    } else {
        1
    };
    let quote = "“".to_string() + quote.as_str() + "“";
    let quote_split = quote.split(" ").map(|c| c.split("\\n")).flatten();
    let mut chunks = Vec::new();
    chunks.push(quote_split.clone().collect::<Vec<&str>>()[0].to_string());
    for chunk in quote_split.skip(1) {
        let char_count = chunk.chars().count();
        let mut line = chunks.pop().unwrap_or("".to_string());
        let line_char_count = line.chars().count();
        let line_size =
            (line_char_count + char_count) * font_setting.get_scale().1 as usize * multibyte_weight;
        if line_size > crate::WIDTH {
            chunks.push(line);
            chunks.push(" ".to_string() + chunk);
        } else {
            line = line + " " + chunk;
            chunks.push(line);
        }
    }
    draw_texts_on_image(image, x, y, chunks, &font_setting);
}
