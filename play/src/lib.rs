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
        let ggg = input.as_bytes().windows(2).enumerate()
                     .find(|(i, c)| (c[0] == b'q' && c[1] == b'u'));
        if ggg != None { index = Some(ggg.unwrap().0 + 1); }
        println!("ggg = {:?}", ggg.unwrap().0);
    }

    // let i = input.as_bytes()
    //              .windows(2)
    //              .enumerate()
    //              .find(|&(i, x)| (vowels.contains(&x[0]) || (&x[0] == b'x' ) ));

    println!("i = {:?}", index);

    todo!()
}
