use html2text::from_read;
use reqwest::blocking::get;
use ui::render_terminal;

pub mod ui;

/// Whatever you are searching for.
pub struct Query {
    pub search: String,
    pub language: String,
}

/// Gets HTML from wikipedia.
pub fn fetch_data(query: Query) -> scraper::Html {
    let request = get(format!(
        "https:/{}.wikipedia.org/wiki/{}",
        query.language, query.search
    ))
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&request);

    document
}

/// Displays formatted html that fits an CSS selector.
pub fn display_article(selector: scraper::Selector, request: scraper::Html) {
    let paragraphs = request
        .select(&selector)
        .map(|x| from_read(x.inner_html().as_bytes(), 150));

    for text in paragraphs {
        render_terminal(text);
    }
}
