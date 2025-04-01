use play::*;

fn main() {
    println!("MAIN");
    let mut game = BowlingGame::new();
    let vvv = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 7];
    println!("length = {}", vvv.len());
    println!("pins = {:?}", vvv);
    for t in vvv {
        match game.roll(t) {
            Ok(()) => (),
            Err(e) => println!("Raphi, the error is {:?}", e)
        }
    }
    println!("Game = {:#?}", game);
    println!("SCore = {:?}", game.score());
}
