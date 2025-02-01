pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut leftover = n;
    let mut divider = 2;
    let limit = (n as f64).sqrt() as u64;

    while divider <= limit {
        while leftover % divider == 0 {
            factors.push(divider);
            leftover /= divider;
        }
        divider += 1;
    }

    if leftover > 1 { factors.push(leftover); }

    factors
}