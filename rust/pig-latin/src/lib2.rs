pub fn translate(input: &str) -> String {

    let words = input.to_ascii_lowercase();
    let words = words.split(' ');
    let mut s: String = String::new();

    for word in words {
        s += &convert(word);
        s.push(' ');
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
    else if qu.is_some() && qu < first_vowel {
        index = Some(qu.unwrap() + 2);
    }
    else if (y < first_vowel || first_vowel.is_none()) && y != Some(0) && y.is_some() {
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