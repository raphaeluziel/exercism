pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v = vec![vec![0; size as usize]; size as usize];

    if size == 0 { return v; }
    if size == 1 { return vec![vec![1]]; }

    let mut direction = 1;
    let mut which = 1;
    let mut n = 1;
    let mut i = 0;
    let mut j = 0;

    while n < size.pow(2) {
        println!("i = {i}, j = {j}");
        v[i as usize][j as usize] = n;
        n += 1;
        if which > 0 { i += direction } else { j += direction }
        direction *= -1;
        which *= -1;
    }

    v
}