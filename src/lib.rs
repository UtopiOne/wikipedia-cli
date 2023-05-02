use html2text::from_read;
use reqwest::blocking::get;
use ui::ArticleDisplay;

pub mod filters;
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

    scraper::Html::parse_document(&request)
}

/// Displays formatted html that fits an CSS selector.
pub fn display_article(
    contents_selector: scraper::Selector,
    title_selector: scraper::Selector,
    request: scraper::Html,
) {
    let paragraphs = filters::remove_square_brackets(
        request
            .select(&contents_selector)
            .map(|x| from_read(x.inner_html().as_bytes(), 190))
            .collect::<Vec<String>>()
            .join("\n"),
    );

    println!("{}", paragraphs);

    let title = request
        .select(&title_selector)
        .map(|x| from_read(x.inner_html().as_bytes(), 50))
        .collect::<Vec<String>>()
        .join("");

    let article = ArticleDisplay {
        title: title,
        contents: paragraphs,
    };

    // ui::ArticleDisplay::new(article).unwrap();
}
