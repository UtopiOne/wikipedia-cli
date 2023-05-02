use html2text::from_read;
use reqwest::blocking::get;

pub struct Query {
    pub search: String,
    pub language: String,
}

pub fn fetch_data(query: Query) {
    let request = get(format!(
        "https:/{}.wikipedia.org/wiki/{}",
        query.language, query.search
    ))
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&request);
    let introduction_paragraph_selector =
        scraper::Selector::parse("div.mw-parser-output>p").unwrap();

    display_article(introduction_paragraph_selector, document);
}

fn display_article(selector: scraper::Selector, request: scraper::Html) {
    let paragraphs = request
        .select(&selector)
        .map(|x| from_read(x.inner_html().as_bytes(), 150));

    for i in paragraphs {
        println!("{i}");
    }
}
