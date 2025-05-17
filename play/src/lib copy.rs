pub fn translate(input: &str) -> String {

    let mut input = input.to_ascii_lowercase();

    let mut input = "square".to_string();   // aresquay

    let vowel = ['a', 'e', 'i', 'o', 'u'];
    let rule_1 = ["a", "e", "i", "o", "u", "xr", "yt"];
    let rule_2 = ["a", "e", "i", "o", "u"];

    if rule_1.iter().any(|&x| input.starts_with(x)) { 
        input += "ay";
    }
    else if !rule_2.iter().any(|&x| input.starts_with(x)) && !input.starts_with("qu") {
        println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");
        let i = input.find(vowel).unwrap_or_default();
        input = input.get(i..).unwrap().to_owned() + input.get(0..i).unwrap() + "ay";
    }
    else if !rule_2.iter().any(|&x| input.starts_with(x)) && input.contains("qu") {
        let mut i = input.find(vowel).unwrap_or_default(); println!("I = {i}");
        let j = input.find("qu").unwrap(); println!("J = {j}");
        i = if i < j { i } else { 0 };
        input = input.get((j+2)..).unwrap().to_owned() + input.get(i..j).unwrap() + input.get(0..i).unwrap() + "quay";
    }

    println!("CCC = {:?}", input);

    input
}
