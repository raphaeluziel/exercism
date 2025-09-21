use std::collections::BTreeMap;

pub fn tally(match_results: &str) -> String {
    let mut scores: BTreeMap<&str, (usize, usize, usize)> = BTreeMap::new();

    println!("{:?}", match_results);

    for line in match_results.split('\n') {
        let ccc = line.split(';').collect::<Vec<&str>>();
        match ccc[2] {
            "win" => {
                let ggg = scores.insert(ccc[0], ());
                scores.entry(ccc[0]).and_modify(|x| x.0 += 1);
                scores.entry(ccc[1]).and_modify(|x| x.1 += 1);
                println!("WIN");
            },
            "loss" => {
                scores.entry(ccc[0]).and_modify(|x| x.1 += 1);
                scores.entry(ccc[1]).and_modify(|x| x.0 += 1);
                println!("LOSS");
            },
            "draw" => {
                scores.entry(ccc[0]).and_modify(|x| x.2 += 1);
                scores.entry(ccc[1]).and_modify(|x| x.2 += 1);
                println!("DRAW");
            },
            _ => {}
        }
    }

    println!("SCORES\n\n{:?}", scores);

    todo!()
}
