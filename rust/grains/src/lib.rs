pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 { panic!("Number of squares must be from 1 to 64"); }
    2u64.pow(s-1)
}

pub fn total() -> u64 {
    // Since a u64 has 64 bits, the maximum it can have is also the answer
    // to the question
    u64::MAX
}