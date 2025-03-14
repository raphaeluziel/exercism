use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = "HE + SEES + THE == LIGHT";
    let xx = input.replace(" ", "").replace("==", "=");
    let mut hh:HashMap<char, u8> = HashMap::new();

    // let mut hh: HashMap<char, u8> = xx
    //     .char_indices()
    //     .filter(|x| x.1 != '+' && x.1 != '=')
    //     .map(|x| (x.1, 0u8))
    //     .collect();

    for x in xx.char_indices() {
        if x.1 != '+' && x.1 != '=' { hh.insert(x.1, 0); }
    }

    hh.insert(input.chars().next().unwrap_or_default(), 1);

    println!("Hash = {:?}", hh.len());



    todo!("Solve the alphametic {input:?}")
}
