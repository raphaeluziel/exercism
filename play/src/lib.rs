pub fn translate(input: &str) -> String {

    let mut input = input.to_ascii_lowercase();

    let mut input = "square".to_string();   // aresquay

    let vowel = ['a', 'e', 'i', 'o', 'u'];
    let rule_1 = ["a", "e", "i", "o", "u", "xr", "yt"];
    let rule_2 = ["a", "e", "i", "o", "u"];

    if rule_1.iter().any(|&x| input.starts_with(x)) { 
        input += "ay";
    }
    else if !rule_2.iter().any(|&x| input.starts_with(x))  && !input.contains("qu") {
        let i = input.find(vowel).unwrap();
        input = input.get(i..).unwrap().to_owned() + input.get(0..i).unwrap() + "ay";
    }
    else if !rule_2.iter().any(|&x| input.starts_with(x)) && input.contains("qu") {
        println!("HERE");
        let i = input.find(vowel).unwrap();
        let j = input.find("qu").unwrap();
        input = input.get(i..j).unwrap().to_owned() + input.get(0..i).unwrap() + "quay";
    }

    println!("CCC = {:?}", input);

    input
}
