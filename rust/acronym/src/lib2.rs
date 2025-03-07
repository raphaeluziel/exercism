// Second iteration using iterators

pub fn abbreviate(phrase: &str) -> String {
    let trimmed_phrase_bytes = phrase.trim().as_bytes();
    
    let mut acronym: Vec<u8> = vec![trimmed_phrase_bytes[0]];

    let mut acronym_end:Vec<u8> = trimmed_phrase_bytes
        .windows(2)
        .enumerate()
        .filter(|x| 
            (x.1[1].is_ascii_uppercase() && !x.1[0].is_ascii_uppercase()) ||
            (x.1[1].is_ascii_lowercase() && (x.1[0] == b' ' || x.1[0] == b'-')) &&
            (x.1[1] != b' ' && x.1[1] != b'-')
        )
        .map(|x| x.1[1])
        .collect();

    acronym.append(&mut acronym_end);

    std::str::from_utf8(&acronym).unwrap_or_default().to_ascii_uppercase()
}