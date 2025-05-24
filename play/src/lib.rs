pub fn translate(input: &str) -> String {
    let input = input.to_ascii_lowercase();

    let input = "rhythaaam";
    input.split(' ')
         .fold(String::new(), |s, word| s + &convert(word) + " ")
         .trim()
         .to_string()
}

fn convert(input: &str) -> String {

    const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let first_vowel = input.char_indices()
                           .find(|x| VOWEL.contains(&x.1))
                           .unwrap_or_default();

    let mut i = first_vowel;

    if !input.starts_with(VOWEL) 
        && !input.starts_with("xr") 
        && !input.starts_with("yt") {

            let fv = [b'a', b'e', b'i', b'o', b'u'];
    let inp = "rhythm";
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
                    ch[0] == b'q' && ch[1] == b'u' ||
                    ch[0] == b'y' && i < first_vowel.0
                );
        println!("ccc = {:?}", ccc);

    }

    

    // const vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    // let fv = input.char_indices()
    //               .find(|x| vowels.contains(&x.1))
    //               .unwrap_or_default();
    
    //let y = input.find('y').unwrap_or_default();
    
    // let mut i = fv.0;
    
    // if i > 0 && 
    //     !input.starts_with("xr") && 
    //     !input.starts_with("yt") {

    //         let prev = input.chars().nth(i - 1).unwrap_or_default();
    //         let curr = input.chars().nth(i).unwrap_or_default();
    //         let next = input.chars().nth(i + 1).unwrap_or_default();

    //         println!("zzz = {:?}, {:?}, {:?}", prev, curr, next);
            
    //         if curr == 'u' && prev == 'q' { i += 1; }
    //         //if curr == 'y' && y < fv.0 { i = ; }
            
    // }

    // println!("first vowel = {:?}, index = {:?}", fv, i);

    // input.get(index..).unwrap_or_default().to_owned() 
    //     + input.get(0..index).unwrap_or_default() 
    //     + "ay"; 
    
    todo!()
}
