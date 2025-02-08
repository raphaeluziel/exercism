pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    
    println!("\n\nLIST = {:?}", list);


    for (index, thing) in list.iter().enumerate() {
        println!("TTT {} - {}", index, thing);
    }

    println!("PROVERB\n{proverb}\n\n");

    proverb
}
