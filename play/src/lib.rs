pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let m = minefield[0].as_bytes();

    println!("MINEFIELD\n{:?}", minefield);


    for (r, row) in minefield.iter().enumerate() {
        println!();
        for (c, col) in row.as_bytes().iter().enumerate() {
            println!("(R_{}, C_{}, VAL__{})", r, c, col);

            

        }
    }

    todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:?}\n");
}
