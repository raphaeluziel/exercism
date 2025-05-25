pub fn translate(input: &str) -> String {
    let input = input.to_ascii_lowercase();

    //let input = "myellow";
    input.split(' ')
         .fold(String::new(), |s, word| s + &convert(word) + " ")
         .trim()
         .to_string()
}

fn convert(input: &str) -> String {

    let mut i = 0;

    const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const VOWEL_Y: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

    if input.starts_with('y') ||
       !input.starts_with(VOWEL) &&
       !input.starts_with("xr") &&
       !input.starts_with("yt") {

            let first_vowel = input.char_indices()
                                   .find(|x| VOWEL.contains(&x.1))
                                   .unwrap_or_default();

            i = first_vowel.0;

            //let prev = input.chars().nth(i - 1).unwrap_or_default();
            //let curr = first_vowel.1;
            //let next = input.chars().nth(i + 1).unwrap_or_default();

            //println!("zzz = {:?}, {:?}, {:?}", prev, curr, next);

            if first_vowel.1 == 'u' && 
                input.chars()
                     .nth(i - 1)
                     .unwrap_or_default() == 'q' { i += 1; }

            if first_vowel.1 == 'y' { println!("HERE, {i}"); }

            println!("ccc = {:?}", i);
            
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

    let index = i;
    input.get(index..).unwrap_or_default().to_owned() 
        + input.get(0..index).unwrap_or_default() 
        + "ay" 
    
    //todo!()
}
