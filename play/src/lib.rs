pub fn translate(input: &str) -> String {

    let mut input = input.to_ascii_lowercase();

    let vowel = ['a', 'e', 'i', 'o', 'u'];
    let rule_1 = ["a", "e", "i", "o", "u", "xr", "yt"];
    let rule_2 = ["a", "e", "i", "o", "u"];

    if rule_1.iter().any(|&x| input.starts_with(x)) { 
        input += "ay";
    }
    else if !rule_2.iter().any(|&x| input.starts_with(x)) {
        let vvv:Vec<_> = input.split
    }
    // else if input.find("qu") != None {
    //     let vvv:Vec<_> = input.split("qu").collect();
    //     let ggg = vvv[0].find(vowel);
    //     let mut fff = "";
    //     if ggg != None {
    //         fff = vvv[0].get(0..ggg.unwrap()).unwrap();
    //         println!("FFF = {:?}", fff);
    //     }
    //     println!("vvv = {:?}", vvv);
    //     input = vvv[1].to_owned() + fff + "qu" + "ay";
    // }

    println!("CCC = {:?}", input);

    input
}
