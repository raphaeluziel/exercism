pub fn recite(start_bottles: u32, take_down: u32) -> String {
	let mut song = String::new();
	let num = ["No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
	
	for i in 0..take_down {
		song += &format!(
			"{0} green bottle{1} hanging on the wall,\n\
			 {0} green bottle{1} hanging on the wall,\n\
			 And if one green bottle should accidentally fall,\n\
			 There'll be {2} green bottle{3} hanging on the wall.\n\n",
			 num[(start_bottles - i) as usize], 
			 if start_bottles - i != 1 { "s" } else { "" },
			 num[(start_bottles - i - 1) as usize].to_ascii_lowercase(), 
			 if start_bottles - i - 1 != 1 { "s" } else { "" });
	}
	song
}