pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = 7;

    let mut v = vec![vec![0; size as usize]; size as usize];

    if size == 0 { return v; }
    if size == 1 { return vec![vec![1]]; }

    let last = size * size;
    let mut count = 1u32;
    let mut start = 0;
    let mut end = size as usize;
    let mut i = 0usize;
    let mut j = 0usize;

    while count < last {
        println!(">>>>>>>");
        while i < end {
            println!("i = {i}, j = {j}, count = {count}");
            v[i][j] = count;
            count += 1;
            i += 1;
        }
        j += 1;
        i -= 1;
        println!("VVVVVVV");
        while j < end {
            println!("i = {i}, j = {j}, count = {count}");
            v[i][j] = count;
            count += 1;
            j += 1;
        }
        j -= 2;
        i -= 1;
        println!("<<<<<<<");
        while i > start {
            println!("i = {i}, j = {j}, count = {count}");
            v[i][j] = count;
            count += 1;
            i -= 1;
        }
        j -= 1;
        i += 1;
        println!("AAAAAAA");
        while j > start {
            println!("i = {i}, j = {j}, count = {count}");
            v[i][j] = count;
            count += 1;
            j -= 1;
        }
        i -= 1;
    }

    v
}