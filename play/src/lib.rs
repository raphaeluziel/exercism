pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    
    println!("\n\nLIST = {:?}", list);

    let mut list_iter = list.iter().peekable();

    while list_iter.peek() != None {
        proverb.push_str(list_iter.next().unwrap());
    }

    list_iter.map(|x| )

    println!("PROVERB\n{proverb}\n\n");
    println!("LIST ITER {:?}", list_iter);

    proverb
}
