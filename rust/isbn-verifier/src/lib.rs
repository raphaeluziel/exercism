/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // If the isbn string is empty return false
    if isbn.is_empty() { return false; }

    // Remove the dashes
    let isbn = isbn.replace("-", "");

    // Any invalid characters (not a digit, a '-' at or an 'X' at end)?
    if !isbn.char_indices().all(|(i, ch)| ch.is_ascii_digit() || (ch == 'X' && i == 9))
    { return false; }

    // If the isbn is too long or too short, it is invalid
    if isbn.len() != 10 { return false; }

    isbn.char_indices()
        // Apply the formula given at:
        // https://en.wikipedia.org/wiki/ISBN#ISBN-10_check_digit_calculation
        // Note the unwrap_or(10)
        // If the last value is an 'X', .to_digit(10) will return None
        // If that happens, use the value 10
        .fold(0, |acc, (i, ch)| { acc + (i as u32 + 1) * ch.to_digit(10).unwrap_or(10) }) % 11 == 0
}