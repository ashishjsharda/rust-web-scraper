use reqwest;
use scraper::{Html, Selector};

fn main() {

    let url = "https://www.rust-lang.org";

    // Call the scrape function
    if let Err(e) = scrape(url) {
        println!("Scraping error: {}", e);
    }
}

fn scrape(url: &str) -> Result<(), reqwest::Error> {
    // Create a client with a User-Agent header
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()?;


    let html = client.get(url).send()?.text()?;

    // Parse the HTML
    let document = Html::parse_document(&html);

    // Create a Selector to find <h2> tags
    let selector = Selector::parse("h2").unwrap();

    // Iterate over each <h2> tag and print its text content
    for element in document.select(&selector) {
        let title = element.text().collect::<Vec<_>>().join("");
        println!("Article title: {}", title);
    }

    Ok(())
}
// The output of the program will be the titles of the articles on the Rust website. 
