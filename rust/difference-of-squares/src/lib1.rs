pub fn square_of_sum(n: u32) -> u32 {
    let mut square_of_sum = 0;
    for i in 1..= n {
        square_of_sum += i;
    }
    square_of_sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum_of_squares: u32 = 0;
    for i in 1..= n {
        sum_of_squares += i * i;
    }
    sum_of_squares
}

pub fn difference(n: u32) -> u32 {
   square_of_sum(n) - sum_of_squares(n)
}