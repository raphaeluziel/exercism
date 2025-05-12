pub fn translate(input: &str) -> String {
    
    // let input = "yttria";

    let mut input = input.to_ascii_lowercase();

    let vowels = ["a", "e", "i", "o", "u"];
    let rule_1 = ["a", "e", "i", "o", "u", "xr", "yt"];

    if rule_1.iter().any(|&x| input.starts_with(x)) { 
        input += "ay";
    }
    println!("CCC = {:?}", input);

    //else if 

    input
}
