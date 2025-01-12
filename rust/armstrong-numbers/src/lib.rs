pub fn is_armstrong_number(num: u32) -> bool {
    let mut x:u32 = num;
    let divisor = 10;
    let mut digits = Vec::new();

    while x != 0 {
        digits.push(x % divisor);
         x /= 10;
    }

    let mut armstrong_number = 0;
    let num_digits = digits.len();

    for d in &digits {
        armstrong_number += d.pow(num_digits as u32);
    }
    
    armstrong_number == num
}