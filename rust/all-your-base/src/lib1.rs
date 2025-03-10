#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 { return Err(Error::InvalidInputBase); }
    if to_base < 2 { return Err(Error::InvalidOutputBase); }

    let mut decimal = 0;
    let mut output_vec:Vec<u32> = Vec::new();
    let mut numvec = number.to_vec();
    numvec.reverse();

    for digit in numvec.iter().enumerate() {
        if digit.1 >= &from_base { return Err(Error::InvalidDigit(*digit.1)) };
        decimal += digit.1 * from_base.pow(digit.0 as u32);
    }
    
    if number.is_empty() || decimal == 0 { output_vec.push(0); }

    while decimal > 0 {
        output_vec.push(decimal % to_base);
        decimal /= to_base;
    }

    output_vec.reverse();
    Ok(output_vec)
}