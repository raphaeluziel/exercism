// AFTER looking at the community solutions, I saw that the while loop I was using
// could be done with an iterator.  The execution time is about the same.

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None; }
    if num == 1 { return Some(Classification::Deficient); }

    let mut sum: u64 = (2..=(num / 2)).filter(|i| num % i == 0).collect::<Vec<u64>>().iter().sum();
    sum += 1;

    match sum.cmp(&num) {
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
        std::cmp::Ordering::Less => Some(Classification::Deficient),
    }
    
}