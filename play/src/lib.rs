pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let vs: Vec<String> = Vec::new();
    let m = minefield[0].as_bytes();
    let stlen = minefield[0].len();
    println!("STLEN {:?}", stlen);


    println!("M -------------------------------> {:?}", m);

    println!("MINEFIELD\n{:?}", minefield);


    for (r, row) in minefield.iter().enumerate() {
        println!();
        // for (c, col) in row.as_bytes().iter().enumerate() {
        //     println!("(R_{}, C_{}, VAL__{})", r, c, col);
        // }

        

    }

    vs
}
