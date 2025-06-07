pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {

    let input = &[vec![9, 8, 7, 8, 9, 8], vec![5, 3, 5, 1, 5, 2], vec![6, 6, 7, 1, 1, 1]];

    let num_rows = input.len();
    let num_cols = input[0].len();

    println!("{num_rows} rows, {num_cols} columns");

    for j in 0..num_cols {
        for i in 0..num_rows {
            println!("({i}, {j}) -> {:?}", input[i][j]);
        }
        println!();
    }

    //let columns: Vec<Vec<u64>> = Vec::with_capacity(input.len());

    // for row in input {
    //     columns.push();
    // }

    // for row in input {
    //     let max = row.iter().max().unwrap();

    //     println!("row = {:?}, max = {:?}", row, max);

    //     let mut columns: Vec<usize> = Vec::new();

    //     let ggg: Vec<_> = row.iter().enumerate()
    //                          .filter(|&x| x.1 == max).map(|x| x.0)
    //                          .collect();

    //     println!("ggg = {:?}", ggg);

        
    //     for col in row.iter().enumerate().filter(|x| x.1 == max).map(|x| x.0) {
    //         println!("col = {:?}", col);

    //     }
    // }
    
    todo!("find the saddle points of the following matrix: {input:?}")
}
