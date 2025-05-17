pub fn translate(input: &str) -> String {

    let input = input.to_ascii_lowercase();
    let input = "square";

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut index: Option<usize> = None;

    if input.starts_with(vowels) 
        || input.starts_with("xr") 
        || input.starts_with("yt") { 
            index = Some(0); 
    }
    else { 
        index = input.find(vowels);
        if index != None {

        }
    }

    // let i = input.as_bytes()
    //              .windows(2)
    //              .enumerate()
    //              .find(|&(i, x)| (vowels.contains(&x[0]) || (&x[0] == b'x' ) ));

    println!("i = {:?}", index);

    todo!()
}
