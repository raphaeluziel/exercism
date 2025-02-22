pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let num = ["Zero", "One", "Two", "Three", "Fouir", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];

    let last_bottle = start_bottles - take_down;
    println!("Start     {start_bottles}\nTakedown  {take_down}\nLast      {last_bottle}");

    let i = 2;

    let s1 = if i > 1 { "s" } else { "" };
    let s2 = if i > 2 { "s" } else { "" };

    let song = format!("{0} green bottle{1} hanging on the wall,\n\
                        {0} green bottle{1} hanging on the wall,\n\
                        And if one green bottle should accidentally fall,\n\
                        There'll be {2} green bottle{3} hanging on the wall.\n\n", 
                        num[start_bottles as usize], s1,
                        num[start_bottles as usize - 1].to_ascii_lowercase(), s2
                      );
                          
    println!("SONG\n\n{song}--------------------------");

    song
}