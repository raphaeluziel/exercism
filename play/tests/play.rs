use play::*;

#[test]
fn lowercase_letter() {
    let input = "a";
    let output = score(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn uppercase_letter() {
    let input = "A";
    let output = score(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn valuable_letter() {
    let input = "f";
    let output = score(input);
    let expected = 4;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn short_word() {
    let input = "at";
    let output = score(input);
    let expected = 2;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn short_valuable_word() {
    let input = "zoo";
    let output = score(input);
    let expected = 12;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn medium_word() {
    let input = "street";
    let output = score(input);
    let expected = 6;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn medium_valuable_word() {
    let input = "quirky";
    let output = score(input);
    let expected = 22;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn long_mixed_case_word() {
    let input = "OxyphenButazone";
    let output = score(input);
    let expected = 41;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn english_like_word() {
    let input = "pinata";
    let output = score(input);
    let expected = 8;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn empty_input() {
    let input = "";
    let output = score(input);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn entire_alphabet_available() {
    let input = "abcdefghijklmnopqrstuvwxyz";
    let output = score(input);
    let expected = 87;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn non_english_scrabble_letters_do_not_score() {
    let input = "piñata";
    let output = score(input);
    let expected = 7;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn german_letters_do_not_score() {
    let input = "STRAßE";
    let output = score(input);
    let expected = 5;
    assert_eq!(output, expected);
}


// #[test]
// #[ignore]
// fn zero() {
//     let input = 0;
//     let output = encode(input);
//     let expected = "zero";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn one() {
//     let input = 1;
//     let output = encode(input);
//     let expected = "one";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn fourteen() {
//     let input = 14;
//     let output = encode(input);
//     let expected = "fourteen";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn twenty() {
//     let input = 20;
//     let output = encode(input);
//     let expected = "twenty";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn twenty_two() {
//     let input = 22;
//     let output = encode(input);
//     let expected = "twenty-two";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn thirty() {
//     let input = 30;
//     let output = encode(input);
//     let expected = "thirty";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn ninety_nine() {
//     let input = 99;
//     let output = encode(input);
//     let expected = "ninety-nine";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn one_hundred() {
//     let input = 100;
//     let output = encode(input);
//     let expected = "one hundred";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn one_hundred_twenty_three() {
//     let input = 123;
//     let output = encode(input);
//     let expected = "one hundred twenty-three";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn two_hundred() {
//     let input = 200;
//     let output = encode(input);
//     let expected = "two hundred";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn nine_hundred_ninety_nine() {
//     let input = 999;
//     let output = encode(input);
//     let expected = "nine hundred ninety-nine";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn one_thousand() {
//     let input = 1_000;
//     let output = encode(input);
//     let expected = "one thousand";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn one_thousand_two_hundred_thirty_four() {
//     let input = 1_234;
//     let output = encode(input);
//     let expected = "one thousand two hundred thirty-four";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn one_million() {
//     let input = 1_000_000;
//     let output = encode(input);
//     let expected = "one million";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn one_million_two_thousand_three_hundred_forty_five() {
//     let input = 1_002_345;
//     let output = encode(input);
//     let expected = "one million two thousand three hundred forty-five";
//     assert_eq!(output, expected);
// }

// #[test]
// //#[ignore]
// fn one_billion() {
//     let input = 1_000_000_000;
//     let output = encode(input);
//     let expected = "one billion";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn a_big_number() {
//     let input = 987_654_321_123;
//     let output = encode(input);
//     let expected = "nine hundred eighty-seven billion six hundred fifty-four million three hundred twenty-one thousand one hundred twenty-three";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn max_i64() {
//     let input = 9_223_372_036_854_775_807;
//     let output = encode(input);
//     let expected = "nine quintillion two hundred twenty-three quadrillion three hundred seventy-two trillion thirty-six billion eight hundred fifty-four million seven hundred seventy-five thousand eight hundred seven";
//     assert_eq!(output, expected);
// }

// #[test]
// #[ignore]
// fn max_u64() {
//     let input = 18_446_744_073_709_551_615;
//     let output = encode(input);
//     let expected = "eighteen quintillion four hundred forty-six quadrillion seven hundred forty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen";
//     assert_eq!(output, expected);
// }
