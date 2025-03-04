pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let phrase = "Something - I made up from thin air";
    for c in phrase.chars() {
        if c.is_uppercase() { acronym.push(c); }
    }
    println!("Acronym = {acronym}");
    acronym
}