pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v = vec![vec![0; size as usize]; size as usize];

    if size == 0 { return v; }
    if size == 1 { return vec![vec![1]]; }



    







    let mut n = 1i32;
    let mut i = 0i32;
    let mut j = 0i32;
    let max: i32 = (size * size) as i32;

    let mut x = 0;
    let a: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut m = (size - 1) as i32; 

    println!("size = {size}");
    while n < max {
        //println!("i = {i}, j = {j}");
        //println!("x = {x}");
        v[i as usize][j as usize] = n as u32;
        n += 1;

        i += a[x].0;
        j += a[x].1;
        
        if (i == m && j < m) || (j == m && i < m) {
            x = if x == 3 { 0 } else { x + 1 };
            m -= 1;
        }
    }

    v
}