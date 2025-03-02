// Trying to get used to iterators, after looking at other solutions

pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect()
}
