// AFTER looking at the community solutions, I saw that if you start the iterator at 1,
// which obviously gets included by the filter predicate, I don't have to sum += 1

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None; }
    if num == 1 { return Some(Classification::Deficient); }

    let sum: u64 = (1..=(num / 2)).filter(|i| num % i == 0).collect::<Vec<u64>>().iter().sum();

    match sum.cmp(&num) {
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
        std::cmp::Ordering::Less => Some(Classification::Deficient),
    }
    
}