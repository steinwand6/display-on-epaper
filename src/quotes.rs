use rand::{seq::SliceRandom, thread_rng};

use serde::{Deserialize, Serialize};

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

pub fn display_quote(file_path: &str) {
    let quotes = get_quotes(file_path);
    let quote = quotes.choose(&mut thread_rng()).unwrap();
    let quote = quote.quote.split("\\n");
    for line in quote {
        println!("{}", line);
    }
}
