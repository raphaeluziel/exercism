pub fn encode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s; }

    let source = "AAAAABBBBSSSUUIAAAAAAAAAAAAAAAAAAAAAAAN";

    let mut ggg = source.as_bytes().windows(2);

    for _ in 0..10 {
        let iii = ggg.by_ref()
                         //.inspect(|x| println!("X = {:?}", x))
                         .take_while(|x| x[0] == x[1])
                         .inspect(|x| println!("X = {:?}", x))
                         .count();
        s += &((iii+1).to_string() + &"-".to_string());
        println!("iii = {:?}", iii+1);
    }
    
    println!("s = {:?}", s);

    // println!("ggg = {:?}", ggg);

    // let jjj = ggg.by_ref()
    //                      //.inspect(|x| println!("X = {:?}", x))
    //                      .take_while(|x| x.1[0] == x.1[1])
    //                      .inspect(|x| println!("Y = {:?}", x))
    //                      .count();

    // println!("ggg = {:?}", ggg);

    // let lll = ggg.by_ref()
    //                      //.inspect(|x| println!("X = {:?}", x))
    //                      .take_while(|x| x.1[0] == x.1[1])
    //                      .inspect(|x| println!("Z = {:?}", x))
    //                      .count();

    // println!("ggg = {:?}", ggg);

    // let mmm = ggg.by_ref()
    //                      //.inspect(|x| println!("X = {:?}", x))
    //                      .take_while(|x| x.1[0] == x.1[1])
    //                      .inspect(|x| println!("M = {:?}", x))
    //                      .count();

    // println!("ggg = {:?}", ggg);
    // println!();

    // println!("\nSource = {:?}\niii = {:?}\njjj = {:?}\nlll = {:?}\nmmm = {:?}", source, iii + 1, jjj + 1, lll + 1, mmm + 1);
    
    // let mut letters = source.chars();
    // let mut n: u64 = 0;
    // let mut curr = letters.next().unwrap_or_default();

    // for letter in letters {
    //     n += 1;
    //     if letter != curr {
    //         if n > 1 { s.push_str(&n.to_string()) }
    //         s.push(curr);
    //         curr = letter;
    //         n = 0;
    //     }
    // }
    // n += 1;
    // if n > 1 { s.push_str(&n.to_string()) }
    // s.push(curr);
    // s
    todo!()
}

pub fn decode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s; }
    
    let letters = source.chars();
    let mut n = 0;
    let mut power = 1;

    for letter in letters {
        if letter.is_ascii_digit() {
            n = power * n + letter.to_digit(10).unwrap_or_default();
            power *= 10;   
        }
        else {
            if n == 0 {
                n = 1;
            }
            for _ in 0..n {
                s.push(letter);
            }
            power = 1;
            n = 0;
        } 
    }

    s
}
