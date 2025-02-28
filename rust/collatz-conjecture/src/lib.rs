pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 { return None; }
    if n == 1 { return Some(0); }
    
    let mut steps = 0;
    let mut num = n;

    while num != 1 {
        num = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
        steps += 1;
    }
    
    Some(steps)
}
