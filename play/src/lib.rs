#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {

    if from_base < 2 { return Err(Error::InvalidInputBase); }
    if to_base < 2 { return Err(Error::InvalidOutputBase); }

    // let number = &[2, 10];
    let mut numvec = number.to_vec();
    numvec.reverse();
    // let from_base = 16u32;
    // let to_base = 3u32;
    // let output_digits = vec![1, 1, 2, 0];

    let mut decimal = 0;

    for digit in numvec.iter().enumerate() {
        println!("digit = {:?}", digit);
        if digit.1 >= &from_base { return Err(Error::InvalidDigit(*digit.1))};
        decimal += digit.1 * from_base.pow(digit.0 as u32);
    }

    let mut ooo:Vec<u32> = Vec::new();
    if number.is_empty() || decimal == 0 { ooo.push(0); }

    let mut i = 0;
    let mut num = decimal;

    while num > 0 {
        ooo.push(num % to_base);
        num /= to_base;
    }

    

    println!("Deciaml = {decimal}");
    // println!("OUTPUT = {:?}", ooo);

    // println!("Number = {}", number[0]*from_base.pow(1) 
    //                       + number[1]*from_base.pow(0));

    // println!("Output = {}", output_digits[0]*to_base.pow(3) 
    //                       + output_digits[1]*to_base.pow(2) 
    //                       + output_digits[2]*to_base.pow(1) 
    //                       + output_digits[3]*to_base.pow(0));

    //todo!("Convert {number:?} from base {from_base} to base {to_base}")
    ooo.reverse();
    Ok(ooo)
}
