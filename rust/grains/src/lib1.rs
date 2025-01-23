pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 { panic!("Number of squares must be from 1 to 64"); }
    let mut num: u64 = 1;
    for _ in 2..=s {
        num *= 2;
    }
    num
}

pub fn total() -> u64 {
    let mut total = 0u64;
    for i in 1..=64 {
        total += square(i);
    }
    total
}