pub fn reverse(input: &str) -> String {
    let s = input.chars().rev();
    let mut r = String::new();
    for c in s {
        r.push(c);
    }
    r
}

#[test]
fn an_empty_string() {
    let input = "";
    let output = reverse(input);
    let expected = "";
    assert_eq!(output, expected);
}

#[test]
// #[ignore]
fn a_word() {
    let input = "robot";
    let output = reverse(input);
    let expected = "tobor";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_capitalized_word() {
    let input = "Ramen";
    let output = reverse(input);
    let expected = "nemaR";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_sentence_with_punctuation() {
    let input = "I'm hungry!";
    let output = reverse(input);
    let expected = "!yrgnuh m'I";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_palindrome() {
    let input = "racecar";
    let output = reverse(input);
    let expected = "racecar";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn an_even_sized_word() {
    let input = "drawer";
    let output = reverse(input);
    let expected = "reward";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn wide_characters() {
    let input = "子猫";
    let output = reverse(input);
    let expected = "猫子";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
#[cfg(feature = "grapheme")]
fn grapheme_cluster_with_pre_combined_form() {
    let input = "Würstchenstand";
    let output = reverse(input);
    let expected = "dnatsnehctsrüW";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
#[cfg(feature = "grapheme")]
fn grapheme_clusters() {
    let input = "ผู้เขียนโปรแกรม";
    let output = reverse(input);
    let expected = "มรกแรปโนยขีเผู้";
    assert_eq!(output, expected);
}

