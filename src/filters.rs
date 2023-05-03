/// Code that cleans the article from HTML leftovers.
use regex::Regex;

pub fn remove_square_brackets(text: String) -> String {
    let square_bracket_regex =
        Regex::new(r"\[(?P<link>[a-zA-Z0-9\(\)\-\,[:space:]&=]+)\]").unwrap();
    square_bracket_regex.replace_all(&text, "$link").to_string()
}

pub fn remove_references(text: String) -> String {
    let reference_regex = Regex::new(r"\[+([0-9]+)\]+").unwrap();
    reference_regex.replace_all(&text, "").to_string()
}

pub fn remove_links(text: String) -> String {
    let link_regex = Regex::new(r": [/?/[a-zA-Z0-9-%:/(/)_.//&=]+/]+\n").unwrap();
    let note_regex =
        Regex::new(r": #cite_note-[0-9-]+\n|: #cite_note-[a-zA-Z_-]+-[0-9-]+\n").unwrap();

    let mut new_text = link_regex.replace_all(&text, "").to_string();
    new_text = note_regex.replace_all(&new_text, "").to_string();

    new_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_square_brackets_test() {
        let test_string = "[Statements][9] in [rust] are [separated] by [semicolons]. Macro prints the message to [standart output]".to_string();

        assert_eq!(
            "Statements9 in rust are separated by semicolons. Macro prints the message to standart output".to_string(),
            remove_square_brackets(test_string),
        );
    }

    #[test]
    fn remove_references_test() {
        let test_string =
            "Statements[[9]][8][7][6] in rust[8] are separated by semicolons.".to_string();

        assert_eq!(
            "Statements in rust are separated by semicolons.".to_string(),
            remove_references(test_string)
        );
    }

    #[test]
    fn remove_links_test() {
        let test_string = ": /wiki/Oxide(III)_Peroxide".to_string();

        assert_eq!("".to_string(), remove_links(test_string));
    }
}
