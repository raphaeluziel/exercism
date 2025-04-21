use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
 
    let candidate = candidate.replace("-", "").replace(" ", "").to_lowercase();
    let ggg:Vec<char> = candidate.chars().collect();
    let sss:HashSet<char> = candidate.chars().collect();

    println!("{:?}\n{:?}", ggg, sss);

    ggg.len() == sss.len()
}
