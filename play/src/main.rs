use play::*;

fn main() {
    println!("MAIN\n");
    let mut game = BowlingGame::new();

    for _ in 0..12 {
        let _ = game.roll(10);
    }

    println!("SCORE = {}", game.score().unwrap());

    // println!("Roll 1 {:?}", game.roll(10));
    // println!("{:?}\n", game);

    // println!("Roll 2 {:?}", game.roll(10));
    // println!("{:?}\n", game);

    // println!("Roll 3 {:?}", game.roll(10));
    // println!("{:?}\n", game);

    // println!("Roll 4 {:?}", game.roll(10));
    // println!("{:?}\n", game);

    // println!("Roll 5 {:?}", game.roll(8));
    // println!("{:?}\n", game);
}
