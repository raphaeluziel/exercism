pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let letters = phrase.char_indices();
    let mut white_before = false;
    let mut lower_before = false;

    for letter in letters {
        if letter.0 == 0 { acronym.push_str(&letter.1.to_uppercase().to_string()); }
        if letter.1 == ' ' || letter.1 == '-' {
            white_before = true;
        } else if letter.1.is_lowercase() {
            lower_before = true;
            if white_before {
                acronym.push_str(&letter.1.to_uppercase().to_string());
            }
            white_before = false;
        } else if letter.1.is_uppercase() {
            if white_before || lower_before {
                acronym.push_str(&letter.1.to_uppercase().to_string());
            }
            white_before = false;
            lower_before = false;
        }
    }
    acronym
}
