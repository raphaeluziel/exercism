use std::io::empty;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minemap: Vec<String> = Vec::new();

    let rows = minefield.len();
    let cols = minefield[0].len();

    for r in 0..rows {
        let row = minefield[r].as_bytes();
        for c in 0..cols {
            println!("HEY {:?}", row.get(c));
        }
    }

    minemap
}

// [
//     " *  * ",
//     "  *   ",
//     "    * ",
//     "   * *",
//     " *  * ",
//     "      ",
// ]
