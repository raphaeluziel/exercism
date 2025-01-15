pub fn verse(n: u32) -> String {

    if n == 0 { return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.".to_string(); }

    if n == 1 { return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\n".to_string(); }

    let bottles_left = n - 1;
    let pl1 = if n == 1 { "" } else { "s" };
    let pl2 = if bottles_left == 1 { "" } else { "s" };
    
    format!("{0} bottle{2} of beer on the wall, {0} bottle{2} of beer.\nTake one down and pass it around, {1} bottle{3} of beer on the wall.\n\n", n, bottles_left, pl1, pl2)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    let mut i: i32 = start as i32;
    while i >= end as i32 {
        s += &verse(i as u32);
        i -= 1;
    }
    s
}