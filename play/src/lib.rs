#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let a = [1, 2, 3, 9];
    let b = [0, 0, 1, 2, 3, 9, 7, 6];

    let small_vec = if a.len() <= b.len() { Vec::from(a) } else { Vec::from(b) };
    let big_vec = if a.len() > b.len() { Vec::from(a) } else { Vec::from(b) };

    // if a.len() <= b.len() {
    //     small_vec = Vec::from(a);
    //     big_vec = Vec::from(b);
    // }
    // else {
    //     small_vec = Vec::from(b);
    //     big_vec = Vec::from(a);
    // }

    println!("BIG = {:?}", big_vec);

    let mut start = 0;
    let mut end = 0;


    // for i in 0..si {
    //     println!("{} -- {}", a[i], b[i]);
    //     while a[i] != b[i+start] && start < bi - si - 1 {
    //         start += 1;
    //     }
    // }

    println!("Start {start}");
    println!("End {end}");

    Comparison::Unequal
}
