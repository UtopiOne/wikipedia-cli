/// Code that cleans the article from HTML leftovers.
use regex::Regex;

pub fn remove_square_brackets(text: String) -> String {
    let square_bracket_regex = Regex::new(r"\[(?P<link>[a-zA-Z[:space:]]+)\]").unwrap();
    square_bracket_regex.replace_all(&text, "$link").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_square_brackets_test() {
        let test_string = "[Statements][9] in [rust] are [separated] by [semicolons]. Macro prints the message to [standart output]".to_string();

        assert_eq!(
            "Statements[9] in rust are separated by semicolons. Macro prints the message to standart output".to_string(),
            remove_square_brackets(test_string),
        );
    }
}
