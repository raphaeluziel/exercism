use play::*;

fn main() {
    println!("MAIN\n");
    let mut game = BowlingGame::new();

    println!("{:?}", game.roll(7));
    println!("{:?}", game.roll(6));
}
