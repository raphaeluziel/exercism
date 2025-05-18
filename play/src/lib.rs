pub fn translate(input: &str) -> String {

    let input = input.to_ascii_lowercase();
    //let input = "square";

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut index: Option<usize> = None;

    let qu = input.find("qu");
    let y = input.find('y');
    let first_vowel = input.find(vowels);

    if input.starts_with(vowels) 
        || input.starts_with("xr") 
        || input.starts_with("yt") {
            println!("Rule 1111111111") ;
            index = Some(0); 
    }
    else if qu != None {
        println!("Rule 3333333333");
        index = Some(qu.unwrap() + 2);
    }
    else if y != None && first_vowel != None && y < first_vowel {
        println!("Rule 4444444444");
        index = y
    }
    else {
        println!("Rule 2222222222");
        index = first_vowel
    }

    let mut jjj = 0;
    if index != None { jjj = index.unwrap(); }

    // let index = input.as_bytes()
    //              .windows(2)
    //              .enumerate()
    //              .inspect(|x| println!("x = {:?}", x))
    //              .find(|&(i, x)| 
    //                 (i == 0 && starts_with_vowel_xr_yt(x[0], x[1])) ||
    //                 (consonant_qu(x[0], x[1]))
    //             );

    let s = input.get(jjj..).unwrap_or_default().to_owned() + input.get(0..jjj).unwrap_or_default() + "ay";
    println!("i = {:?}", index);
    println!("String = {:?} + {:?}", input.get(jjj..), input.get(0..jjj));

    //todo!()
    s
}

// fn starts_with_vowel_xr_yt(a: u8, b: u8) -> bool {
//     let vowels = [b'a', b'e', b'i', b'o', b'u'];
//     vowels.contains(&a) || (a == b'x' && b == b'r') || (a == b'y' && b == b't')
// }

// fn consonant_qu(a: u8, b:u8) -> bool {
//     a == b'q' && b == b'u'
// }
