pub fn translate(input: &str) -> String {
    let input = input.to_ascii_lowercase();

    input.split(' ')
         .fold(String::new(), |s, word| s + &convert(word) + " ")
         .trim()
         .to_string()
}

fn convert(input: &str) -> String {
    let index;
    
    let qu = input.find("qu");
    let y = input.find('y');
    let first_vowel = input.find(['a', 'e', 'i', 'o', 'u']);

    let fv = [b'a', b'e', b'i', b'o', b'u'];
    let inp = "square";
    let ccc = inp.as_bytes()
                .windows(2).enumerate()
                .find(|&(i, ch)|
                    // Rule 1 starts with vowel or 'xr' or 'yt'
                    i == 0 && (fv.contains(&ch[0]) || 
                               ch[0] == b'x' && ch[1] == b'r' || 
                               ch[0] == b'y' && ch[1] == b't') ||
                    // Rule 2 starts with consonant(s)
                    fv.contains(&ch[0]) ||
                    // Rule 3 "qu" with no previous vowels
                    ch[0] == b'q' && ch[1] == b'u'
                );
    println!("ccc = {:?}", ccc);

    if first_vowel == Some(0) || 
        input.starts_with("xr") || 
        input.starts_with("yt") {
            index = 0; 
    }
    else if qu.is_some() && qu < first_vowel {
        index = qu.unwrap() + 2;
    }
    else if (y < first_vowel || first_vowel.is_none()) && 
        y > Some(0) && y.is_some() {
        index = y.unwrap_or_default()
    }
    else {
        index = first_vowel.unwrap_or_default()
    }

    println!("ind = {:?}", index);

    // input.get(index..).unwrap_or_default().to_owned() 
    //     + input.get(0..index).unwrap_or_default() 
    //     + "ay"; 
    
    todo!()
}
