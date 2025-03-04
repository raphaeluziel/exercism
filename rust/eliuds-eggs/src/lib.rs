pub fn egg_count(display_value: u32) -> usize {
    let mut x = display_value;
    let mut count = 0;

    while x > 0 {
        if x % 2 == 1 { count += 1; }
        x = x / 2;
    }

    count
}
