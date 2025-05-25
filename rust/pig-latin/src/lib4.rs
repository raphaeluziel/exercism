pub fn translate(input: &str) -> String {
    let input = input.to_ascii_lowercase();

    input.split(' ')
         .fold(String::new(), |s, word| s + &convert(word) + " ")
         .trim()
         .to_string()
}

fn convert(input: &str) -> String {
    let mut i = 0;
    let mut vowels = vec!['a', 'e', 'i', 'o', 'u'];

    if !input.starts_with('y') { vowels.push('y'); }

    if !input.starts_with(vowels.as_slice()) &&
       !input.starts_with("xr") &&
       !input.starts_with("yt") {
            let first_vowel = input.char_indices()
                                   .find(|x| vowels.contains(&x.1))
                                   .unwrap_or_default();

            i = first_vowel.0;

            if first_vowel.1 == 'u' && 
                input.chars()
                     .nth(i - 1)
                     .unwrap_or_default() == 'q' { i += 1; }   
    }

    input.get(i..).unwrap_or_default().to_owned() 
        + input.get(0..i).unwrap_or_default() 
        + "ay" 
}
