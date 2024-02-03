extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use reqwest::blocking::get;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org";
    let response = reqwest::blocking::get(url)?;
    assert!(response.status().is_success());
    let body = response.text()?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();
    for element in fragment.select(&selector) {
        let href = element.value().attr("href").unwrap_or_default();
        println!("{}", href);
    }
    Ok(())
}
