pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut subs: Vec<String> = Vec::new();
    let mut start = 0;

    while start + len <= digits.len() {
        subs.push(digits.get(start..(start + len)).unwrap().to_string());
        start += 1;
    }

    subs
}