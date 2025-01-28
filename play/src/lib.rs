pub fn is_prime(num: u32) -> bool {

    if num < 2 { return false; }
    if num == 2 { return true; }

    let mut x = 3;

    let limit = (num as f32).sqrt() as u32;

    while x < limit {
        if num % x == 0 { return false; }
        x += 2;
    }

    true
}


pub fn nth(n: u32) -> u32 {

    if n == 0 { return 2; }
    else if n == 1 { return 3; }
    
    let mut count = 1;
    let mut num = 5;

    while count < n {
        if is_prime(num) {
            count += 1;
        }
        num += 2;
    }

    println!("COUNT {count}");

    num
}
