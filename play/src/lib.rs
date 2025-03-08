use core::num;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let number = &[2, 10];
    let mut numvec = number.to_vec();
    numvec.reverse();
    let from_base = 16u32;
    let to_base = 3u32;
    let output_digits = vec![1, 1, 2, 0];

    let decimal = 0;

    for digit in numvec {
        println!("digit = {:?}", digit);
    }

    println!("Deciaml = {decimal}");

    println!("Number = {}", number[0]*from_base.pow(1) 
                          + number[1]*from_base.pow(0));

    println!("Output = {}", output_digits[0]*to_base.pow(3) 
                          + output_digits[1]*to_base.pow(2) 
                          + output_digits[2]*to_base.pow(1) 
                          + output_digits[3]*to_base.pow(0));

    todo!("Convert {number:?} from base {from_base} to base {to_base}")
}
