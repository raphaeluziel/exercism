use play::*;
use std::collections::BTreeMap;

fn main() {
    println!("MAIN\n");

    let input = BTreeMap::from([
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ]);

    let expected = BTreeMap::from([
        ('a', 1),
        ('b', 3),
        ('c', 3),
        ('d', 2),
        ('e', 1),
        ('f', 4),
        ('g', 2),
        ('h', 4),
        ('i', 1),
        ('j', 8),
        ('k', 5),
        ('l', 1),
        ('m', 3),
        ('n', 1),
        ('o', 1),
        ('p', 3),
        ('q', 10),
        ('r', 1),
        ('s', 1),
        ('t', 1),
        ('u', 1),
        ('v', 4),
        ('w', 4),
        ('x', 8),
        ('y', 4),
        ('z', 10),
    ]);

    transform(&input);

    //println!("INPUT\n{:?}\n", input);
    //println!("EXPECTED\n{:?}", expected);

    //assert_eq!(expected, play::transform(&input));
}
