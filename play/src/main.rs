use play::*;

fn main() {

    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);

    println!("JJJ\n{:?}", high_scores.latest());
}