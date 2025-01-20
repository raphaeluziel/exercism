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

    // let small_index = if a.len() <= b.len() { a.len() } else { b.len() };
    let mut start = 0;
    let mut end = a.len() - 1;

    println!("Start {} End {}", start, end);

    while (&b[start..end] != a) && (end < b.len()) {
        start += 1;
        end += 1;
    }
    println!("Start {} End {}", start, end);

    // let small_vec = if a.len() <= b.len() {
    //     Vec::from(a)
    // } else {
    //     Vec::from(b)
    // };
    // let big_vec = if a.len() > b.len() {
    //     Vec::from(a)
    // } else {
    //     Vec::from(b)
    // };

    // let mut start = 0;
    // let mut end = 0;

    // for (i, n) in big_vec.iter().enumerate() {
    //     if small_vec.get(start) == Some(n) {
    //         start = i;
    //         big_vec = Vec::from(&b[start..]);
    //         break;
    //     }
    // }

    // println!("Start {start}");
    // println!("BIG = {:?}", big_vec);
    // println!("End {end}");

    // let mut start: usize = 0;
    // let end: usize = big_vec.len();

    // let mut slice_of_big = &big_vec;

    // while (&slice_of_big[start..(start + a.len())] != a) && (start + a.len() < slice_of_big.len()){
    //     println!("HEY");
    //     start += 1;
    // }

    Comparison::Unequal
}
