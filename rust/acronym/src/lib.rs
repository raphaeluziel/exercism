pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::from(phrase.trim().chars().next().unwrap_or_default());

    let acronym_end: String = phrase.trim().as_bytes().windows(2).enumerate()
        .filter(|x|
            (x.1[1].is_ascii_uppercase() && !x.1[0].is_ascii_uppercase()) || 
            (x.1[1].is_ascii_lowercase() && (x.1[0] == b' ' || x.1[0] == b'-')) && 
            (x.1[1] != b' ' && x.1[1] != b'-'))
        .map(|x| x.1[1] as char)
        .collect();

    acronym.push_str(&acronym_end);
    acronym.to_ascii_uppercase()
}