use std::env;
use wikipedia_cli::{display_article, fetch_data, Query};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("To see the article, type what you need as an argument.");
        std::process::exit(1);
    }

    let language = env::var("Q_LANG");
    match language {
        Ok(v) => println!("language: {}", v),
        Err(e) => println!("Error: {e}"),
    }

    let search = &args[1];

    let paragraph_selector = scraper::Selector::parse("div.mw-parser-output>p").unwrap();

    let title_selector = scraper::Selector::parse(".mw-page-title-main").unwrap();

    let document = fetch_data(Query {
        search: search.to_string(),
        language: "en".to_string(),
    });

    display_article(paragraph_selector, title_selector, document);
}
