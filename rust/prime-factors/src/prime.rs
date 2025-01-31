pub fn is_prime(num: u64) -> bool {

    if num < 2 { return false; }

    let mut x = 2;

    let limit = (num as f64).sqrt() as u64;

    while x <= limit {
        if num % x == 0 { return false; }
        x += 1;
    }

    true
}

pub fn nth(n: u64) -> u64 {

    let mut prime = 0;

    if n == 0 { prime = 2; }
    
    let mut count = 1;
    let mut num = 3;

    while count <= n {
        if is_prime(num) {
            count += 1;
            prime = num;
        }
        num += 2;
    }
    prime
}
