pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")

    let mut x:u32 = num;
    let divisor = 10;
    let mut digits = Vec::new();

    while x != 0 {
        digits.push(x % divisor);
         x /= 10;
    }

    let mut armstrong_number = 0;
    let num_digits = digits.len();

    for d in &digits {
        armstrong_number += d.pow(num_digits as u32);
    }

    println!("NUM = {num}");
    println!("ARM = {armstrong_number}");

    println!("TRUE???? {}", armstrong_number == num);
    armstrong_number == num
}







#[test]
fn zero_is_an_armstrong_number() {
    assert!(is_armstrong_number(0))
}

#[test]
#[ignore]
fn single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
#[ignore]
fn there_are_no_two_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
#[ignore]
fn three_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
#[ignore]
fn three_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
#[ignore]
fn four_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_474))
}

#[test]
#[ignore]
fn four_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_475))
}

#[test]
#[ignore]
fn seven_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_926_315))
}

#[test]
#[ignore]
fn seven_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_926_314))
}
