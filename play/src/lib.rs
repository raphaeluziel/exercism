pub fn translate(input: &str) -> String {

    let input = input.to_ascii_lowercase();

    let v = input.find(['a', 'e', 'i', 'o', 'u', 'x', 'y', 'q']);
    println!("{input}: {:?}", v);

    match v {
        Some(0) => println!("Rule 1"),
        
    }

    todo!()
}
