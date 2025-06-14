// const ZEROS: [char; 10] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
// const ONES: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'];
// const TWOS: [char; 10] = ['D', 'G', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
// const THREES: [char; 10] = ['B', 'C', 'M', 'P', ' ', ' ', ' ', ' ', ' ', ' '];
// const FOURS: [char; 10] = ['F', 'H', 'V', 'W', 'Y', ' ', ' ', ' ', ' ', ' '];
// const FIVES: [char; 10] = ['K', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
// const EIGHTS: [char; 10] = ['J', 'K', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
// const TENS: [char; 10] = ['Q', 'Z', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];

// const LETTER_VALUES: [[char; 10]; 11] = [
//     ZEROS, ONES, TWOS, THREES, FOURS, FIVES, ZEROS, ZEROS, EIGHTS, ZEROS, TENS,
// ];

use std::collections::HashMap;

pub fn score(word: &str) -> u64 {
    let ones = "AEIOULNRST";
    let twos = "DG";
    let threes = "BCMP";
    let fiours = "FHVWY";
    let fives = "K";
    let eights = "JX";
    let tens = "QZ";
    
    let ccc: HashMap<char, u64> = ones.chars().map(|ch| 
        match  {
            'A'| 'E'| 'I'| 'O'| 'U'| 'L'| 'N'| 'R'| 'S'| 'T' => 1,

        }
        )
        .collect();
    println!("ccc = {:?}", ccc);

    // word.chars().fold(0, |acc, x| )
    todo!("Score {word} in Scrabble.");
}

// const DIGITS: [&str; 10] = [
//     "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
// ];

// const TENS: [&str; 10] = [
//     "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
// ];

// const TEENS: [&str; 10] = [
//     "ten",
//     "eleven",
//     "twelve",
//     "thirteen",
//     "fourteen",
//     "fifteen",
//     "sixteen",
//     "seventeen",
//     "eighteen",
//     "nineteen",
// ];

// const BREAKERS: [&str; 7] = [
//     "",
//     "thousand",
//     "million",
//     "billion",
//     "trillion",
//     "quadrillion",
//     "quintillion",
// ];

// fn say_triple(n: usize) -> String {
//     if n == 0 { "".to_string() }
//     else if n > 99 { DIGITS[n / 100].to_string() + " hundred " + &say_double(n % 100) }
//     else { say_double(n) }
// }

// fn say_double(n: usize) -> String {
//     match n {
//         0 => "".to_string(),
//         1..10 => DIGITS[n].to_string(),
//         10..20 => TEENS[n - 10].to_string(),
//         x if x % 10 == 0 => TENS[n / 10].to_string(),
//         _ => TENS[n / 10].to_string() + "-" + DIGITS[n % 10]
//     }
// }

// pub fn encode(n: u64) -> String {
//     if n == 0 { return "zero".to_string(); }

//     let mut s = String::new();

//     let mut num = n;
//     let mut threes: Vec<u64> = Vec::new();

//     while num > 0 {
//         threes.push(num % 1000);
//         num /= 1000;
//     }

//     threes.reverse();

//     let mut breaker = threes.len();

//     for triple in &threes {
//         breaker -= 1;
//         if *triple != 0 { s += &(say_triple(*triple as usize) + " " + BREAKERS[breaker] + " ") }
//     }

//     s.trim().to_string()
// }
