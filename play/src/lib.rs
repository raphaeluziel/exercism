pub fn translate(input: &str) -> String {

    let words = input.to_ascii_lowercase();
    let words = words.split(" ");
    let mut s: String = String::new();

    for word in words {
        s += &convert(word);
        s.push_str(" ");
    }

    s.trim().to_string()
}

fn convert(input: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let index;

    let qu = input.find("qu");
    let y = input.find('y');
    let first_vowel = input.find(vowels);

    if input.starts_with(vowels) 
        || input.starts_with("xr") 
        || input.starts_with("yt") {
            index = Some(0); 
    }
    else if qu != None && qu < first_vowel {
        index = Some(qu.unwrap() + 2);
    }
    else if y != None && y != Some(0) && 
        (first_vowel == None || first_vowel != None && y < first_vowel) {
        index = y
    }
    else {
        index = first_vowel
    }

    let mov = index.unwrap_or_default();

    input.get(mov..).unwrap_or_default().to_owned() 
        + input.get(0..mov).unwrap_or_default() 
        + "ay"
}
