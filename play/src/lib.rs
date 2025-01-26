pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minemap: Vec<String> = Vec::new();

    let rows = minefield.len();
    let cols = minefield[0].len();

    let mut vvv = vec![vec![0u8; cols + 2]; rows + 2];
    
    for r in 0..rows {
        let row = minefield[r].as_bytes();

        for c in 0..cols {
            if row[c] == 42u8 { vvv[r + 1][c + 1] += 1; }
        }


    }

    println!("{:?}", vvv);

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
