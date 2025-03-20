use play::*;

fn main() {
    println!("MAIN");
    let answer = solve("NO + NO + TOO == LATE");
    let expected = [('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]
        .into_iter()
        .collect();
    assert_eq!(answer, Some(expected));
    println!("Answer = {:?}", answer);
}
