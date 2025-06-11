const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const BREAKERS: [&str; 7] = [
    "hundred",
    "thousand",
    "million",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
];

pub fn encode(n: u64) -> String {
    let mut s = String::new();
    // let mut n = 18_123_198_090_708_541_630;
    //let n = 1;
    let mut num = n;

    if num < 10 { return DIGITS[num as usize].to_string(); }
    if num < 20 { return TEENS[(num % 10) as usize].to_string(); }

    let mut threes: Vec<u64> = Vec::new();

    while num > 0 {
        threes.push(num % 1000);
        num /= 1000;
    }

    threes.reverse();

    //println!("Digits = {:?}", threes);

    let ddd = 427;
    println!("ddd = {}, {}, {}", ddd/100, ddd/10, ddd%10);



    todo!("Say {n} in English.");
}
