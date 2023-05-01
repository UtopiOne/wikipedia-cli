use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search = &args[1];

    println!("{search}");
}

fn fetch_data(search: &String) -> String {
    todo!();
}
