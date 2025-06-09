pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {

    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    if input[0].is_empty() { return saddle_points; }

    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut mins: Vec<u64> = Vec::new();
    let ccc: Vec<_> = (0..num_cols).collect();
    println!("ccc = {:?}", ccc);
    
    for j in 0..num_cols {
         mins.push(*(0..num_rows).enumerate()
                                 .map(|x| input[x.0][j])
                                 .collect::<Vec<_>>()
                                 .iter().min()
                                 .unwrap());
    }

    for (index, row) in input.iter().enumerate() {
        let max = row.iter().max().unwrap();

        let it = row.iter().enumerate()
                    .filter(|&x| x.1 == max && x.1 == &mins[x.0])
                    .map(|x| x.0);

        for col in it {
            saddle_points.push((index, col));
        }
    }
    todo!()
    //saddle_points
}
