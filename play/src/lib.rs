pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    //let size = 4;

    let mut v = vec![vec![0u32; size as usize]; size as usize];

    if size == 0 {
        return v;
    }
    if size == 1 {
        return vec![vec![1]];
    }

    let last = size * size;
    let mut count = 1u32;
    let mut start = 0usize;
    let mut end = size as usize - 1;

    while count <= last {

        // Move right (do nothing to start or end)
        println!(">>>>>>>");
        for col in start..=end {
            println!("{col}, {start} ---> {count}");
            v[start][col] = count;
            count += 1;
        }

        // Move down (increase start by 1)
        println!("VVVVVVV");
        start += 1;
        for row in start..=end {
            println!("{end}, {row} ---> {count}");
            v[row][end] = count;
            count += 1;
        }

        // Move left (decrease start by 1 and decrease end by 1)
        println!("<<<<<<<");
        start -= 1;
        end -= 1;
        for col in (start..=end).rev() {
            println!("{col}, {} ---> {count}", end + 1);
            v[end + 1][col] = count;
            count += 1;
        }

        // Move up (increase start by 1)
        println!("AAAAAAA");
        start += 1;
        for row in (start..=end).rev() {
            println!("{}, {row} ---> {count}", start - 1);
            v[row][start - 1] = count;
            count += 1;
        }

    }

    println!("\n{:?}", v);

    v
}
