use play::*;

fn main() {
    println!("MAIN");
    let mut game = BowlingGame::new();
    let vvv = vec![6, 2, 10, 3, 5, 2, 2, 10, 10, 7, 3, 1, 1, 6, 4, 7, 3, 2];
    println!("length = {}", vvv.len());
    for t in vvv {
        match game.roll(t) {
            Ok(()) => (),
            Err(e) => println!("Raphi, the error is {:?}", e)
        }
    }
    println!("Game = {:#?}", game.score());
}
