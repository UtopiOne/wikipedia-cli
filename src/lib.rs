use html2text::from_read;
use reqwest::blocking::get;

pub fn fetch_data(search: &String) {
    let request = get(format!("https:/en.wikipedia.org/wiki/{}", search))
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&request);
    let introduction_paragraph_selector =
        scraper::Selector::parse("div.mw-parser-output>p").unwrap();

    let introduction_paragraph = document
        .select(&introduction_paragraph_selector)
        .map(|x| from_read(x.inner_html().as_bytes(), 20));

    introduction_paragraph.for_each(|x| print!("{x}"));
}
