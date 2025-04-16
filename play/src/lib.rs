use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut b:BTreeMap<char, i32> = BTreeMap::new();

    // println!("{:?}", h[&4]);

    let jj = h.iter()
                     .map(|x| x.1.iter().map(move |&y| (y, x.0)).collect::<Vec<_>>());
                     //.inspect(|x| println!("outer = {:?}", x))
                     //.collect();

    println!();

    let gg:Vec<_> = h[&4].iter()
                         .map(|&x| (x, 4))
                         .inspect(|x| println!("inner = {:?}", x))
                         .collect();

    println!("\nSO FAR\n{:?}", jj);

    //let rr = BTreeMap::from(jj);
    //println!("FINAL? {:?}", rr);

    // for k in h {
    //     for x in k.1 {
    //         b.insert((*x).to_ascii_lowercase(), *k.0);
    //     }
    // }



    // let input = BTreeMap::from([
    //     (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
    //     (2, vec!['D', 'G']),
    //     (3, vec!['B', 'C', 'M', 'P']),
    //     (4, vec!['F', 'H', 'V', 'W', 'Y']),
    //     (5, vec!['K']),
    //     (8, vec!['J', 'X']),
    //     (10, vec!['Q', 'Z']),
    // ]);

    b
}
