/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {

    
    
    isbn.chars().rev()
        .filter(|&ch| ch.is_ascii_digit() || ch == 'X')
        .enumerate()
        .inspect(|x| println!("X = {:?}", x))
        .fold(0, |acc, (i, ch)| acc + 
                    (i as u32 + 1) * ch.to_digit(10)
                .unwrap_or(10)) 
        % 11 == 0  
}