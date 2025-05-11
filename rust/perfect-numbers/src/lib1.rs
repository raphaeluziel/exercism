#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None; }
    if num == 1 { return Some(Classification::Deficient); }
    
    let mut factors: Vec<u64> = vec![];
    factors.push(1);

    let mut i = 2;
    while i < num {
        if num % i == 0 { 
            factors.push(i);
        }
        i += 1;
    }

    let sum: u64 = factors.iter().sum();

    match sum.cmp(&num) {
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
        std::cmp::Ordering::Less => Some(Classification::Deficient),
    }
    
}
