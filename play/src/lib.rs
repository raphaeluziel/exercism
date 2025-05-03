use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    palindrome: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        todo!("return the value of the palindrome")
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        todo!("return the set of factors of the palindrome")
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    println!("Min = {min}, Max = {max}");
    let mut bts: BTreeSet<u64> = BTreeSet::new();
    
    for i in min..=max {
        for j in i..=max {
            bts.insert(i * j);
        }
    }

    for n in bts {
        let smallest = bts;
        let smallest;
        //println!("{}", n.to_string());
        let s = n.to_string();
        let s = s.as_bytes();
        //println!("s = {:?}", s);
        let x = s.iter().zip(s.iter().rev()).any(|(f, b)| f != b);
        let y:Vec<_> = s.iter().zip(s.iter().rev()).filter(|(f, b)| f == b).map(|x| x.0 - b'0').collect();
        if !x { 
            println!("{:?}", n);
            
        }
        println!("y = {:?}", y);
        //println!("x = {:?}", x);
        //let ggg = s.zip(s.rev()).any(|(f, b)| f != b);

        //println!("gg = {:?}", ggg);
    }

    //println!("H = {:?}", bts);
    todo!(
        "returns the minimum palindrome and maximum palindrome of the products of two factors in the range {min} to {max}"
    );
}
