use std::env;
use wikipedia_cli::{fetch_data, Query};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter what you are searching for.");
        std::process::exit(1);
    }

    let language = env::var("Q_LANG");
    match language {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error: {e}"),
    }

    let search = &args[1];

    fetch_data(Query {
        search: search.to_string(),
        language: "en".to_string(),
    });
}
