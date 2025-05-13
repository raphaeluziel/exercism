pub fn translate(input: &str) -> String {

    let input = "liquid";

    let mut input = input.to_ascii_lowercase();

    let rule_1 = ["a", "e", "i", "o", "u", "xr", "yt"];

    if rule_1.iter().any(|&x| input.starts_with(x)) { 
        input += "ay";
    }
    else if input.find("qu") != None {
        let vvv:Vec<_> = input.split("qu").collect();
        println!("vvv = {:?}", vvv);
        input = vvv[1].to_owned() + vvv[0] + "qu" + "ay";
    }

    println!("CCC = {:?}", input);

    input
}
