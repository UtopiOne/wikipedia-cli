use html2text::from_read;
use reqwest::blocking::get;
use scraper::Selector;

pub fn fetch_data(search: &String) {
    let request = get(format!("https:/en.wikipedia.org/wiki/{}", search))
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&request);
    let introduction_paragraph_selector =
        scraper::Selector::parse("div.mw-parser-output>p").unwrap();

    display_article(introduction_paragraph_selector, document);
}

fn display_article(selector: Selector, request: scraper::Html) {
    let paragraphs = request
        .select(&selector)
        .map(|x| from_read(x.inner_html().as_bytes(), 150));

    for i in paragraphs {
        println!("{i}");
    }
}
