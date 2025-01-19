use play as squares;

fn main() {
    let n: u32 = 300;
    let x = squares::difference(n);
    println!("Difference using n = {} is {}", n, x);
}