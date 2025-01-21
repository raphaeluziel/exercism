#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let a = [1, 2, 3, 9];
    let b = [990, 880, 1, 2, 32, 9, 110, 220, 330, 440];

    let sm: &[i32] = if a.len() <= b.len() { &a } else { &b };
    let lg: &[i32] = if a.len() > b.len() { &a } else { &b };

    if sm == lg { return Comparison::Equal; }

    let smi = sm.len();
    let lgi = lg.len();

    println!("SMALL {} {:?}", smi, sm);
    println!("LARGE {} {:?}", lgi, lg);

    println!("HEYYYYYYY {}", sm == lg);

    for i in 0..=(lgi - smi) {
        println!("{i}");
        let lg_slice: &[i32] = &lg[i..(i + smi)];
        println!("SLICE = {:?}", lg_slice);
        if sm == lg_slice { println!("\n\nSUBSET!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\n"); }
    }

    Comparison::Unequal
}
