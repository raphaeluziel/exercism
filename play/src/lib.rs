pub fn nth(n: u32) -> u32 {
    // const FIRST_FEW: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    // if n < 13 {
    //     FIRST_FEW[n as usize]
    // }
    // else {
    //     n
    // }

    fn is_prime(num: u32) -> bool {
        
        true
    }

    let count: u32 = 0;
    while count <= n {

    }


    for num in (3..).into_iter().take_while(|x| x < (n as f64).sqrt() as u32) {
        println!("HEY {num}");
    }

    n
}
