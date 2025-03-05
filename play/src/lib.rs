pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let phrase = "Something - I made up from thin air";

    let letters = phrase.char_indices();

    for letter in letters {
        let mut white_before = false;
        let mut lower_before = false;
        println!("Letter = {}", letter.1);
        if letter.1 == ' ' || letter.1 == '-' {
            println!("whitespace");
            white_before = true;
        } else if letter.1.is_lowercase() {
            println!("lowercase");
            lower_before = true;
            if white_before {
                println!("white before");
                acronym.push(letter.1);
            }
        }
        else if letter.1.is_uppercase() {
            println!("uppercase");
            if white_before || lower_before {
                acronym.push(letter.1);
            }
        }
    }

    println!("Acronym = {:?}", acronym);

    acronym
}
