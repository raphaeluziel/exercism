use std::io::empty;

pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let minemap: Vec<String> = Vec::new();
    let empty_slice = &vec![32; minefield[0].len()];
    let mlen = minefield.len();

    for row in 0..mlen {
        let row_above = if row > 0 { minefield[row - 1].as_bytes() } else { empty_slice };
        let row__here = minefield[row].as_bytes();
        let row_below = if row < mlen - 1 { minefield[row + 1].as_bytes() } else { empty_slice };
        //println!("Row {row}\nRow Above = {:?}\nRow       = {:?}\nRow Below = {:?}\n", row_above, row__here, row_below);

        for col in 0..row__here.len() {
            print!("{:?} ", row__here[col]);
            let num_mines:u8 = 0;
            
        }
        println!();

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
