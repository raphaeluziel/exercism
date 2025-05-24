pub fn translate(input: &str) -> String {
    let input = input.to_ascii_lowercase();

    let input = "square";
    input.split(' ')
         .fold(String::new(), |s, word| s + &convert(word) + " ")
         .trim()
         .to_string()
}

fn convert(input: &str) -> String {

    const vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut fv = input.char_indices()
                  .find(|x| vowels.contains(&x.1))
                  .unwrap_or_default();
    
    if fv.0 > 0 && 
        !input.starts_with("xr") && 
        !input.starts_with("yt") {

            let prev = input.chars().nth(fv.0 - 1).unwrap_or_default();
            let curr = input.chars().nth(fv.0).unwrap_or_default();
            let next = input.chars().nth(fv.0 + 1).unwrap_or_default();

            println!("zzz = {:?}, {:?}, {:?}", prev, curr, next);
            
            if curr == 'u' && prev == 'q' {  }
            
            
    }

    println!("ind = {:?}", fv);

    // input.get(index..).unwrap_or_default().to_owned() 
    //     + input.get(0..index).unwrap_or_default() 
    //     + "ay"; 
    
    todo!()
}
