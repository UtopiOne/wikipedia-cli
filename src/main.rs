use html2text::from_read;
use std::env;
use wikipedia_cli::fetch_data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search = &args[1];

    fetch_data(search);
}

fn read_html(html: &[u8]) -> String {
    from_read(&html[..], 20)
}
