use std::mem::take;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    let bottles = ["bottle", "bottles"];

    let num = [
        "Ten", 
        "Nine", 
        "Eight", 
        "Seven", 
        "Six", 
        "Five", 
        "Four", 
        "Three", 
        "Two", 
        "One", 
        "Zero"
        ];

    let last_bottle = start_bottles - take_down;
    println!("Start     {start_bottles}\nTakedown  {take_down}\nLast      {last_bottle}");

    let x = format!("{} green {} hanging on the wall,\n", start_bottles, bottles[1]);
    for _ in 0..2 { song.push_str(&x); }
    song.push_str("And if one green bottle should happen to fall,\n");
    song.push_str();


    println!("SONG\n{}", song);

    // Four green bottles hanging on the wall,
    // Four green bottles hanging on the wall,
    // And if one green bottle should accidentally fall,
    // There'll be three green bottles hanging on the wall.

    song
}