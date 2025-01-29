use core::str;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minemap: Vec<String> = Vec::new();

    if minefield.is_empty() { return minemap; }

    let rows = minefield.len();
    let cols = minefield[0].len();

    let mut matrix_with_border = vec![vec![32u8; cols + 2]; rows + 2];
    let mut matrix_of_values = vec![vec![0u8; cols]; rows];

    // the location (1,1) is the central one, the one we are looking at
    let locations = [(0, 0), (0, 1), (0, 2), 
                     (1, 0),         (1, 2), 
                     (2, 0), (2, 1), (2, 2)];

    for r in 0..rows {
        let row = minefield[r].as_bytes();
        matrix_with_border[r + 1][1..(cols + 1)].copy_from_slice(&row[..cols]);
    }

    for r in 0..rows {
        for c in 0..cols {

            if matrix_with_border[r + 1][c + 1] == 42 {
                matrix_of_values[r][c] = 42;
                continue;
            }

            for loc in locations {
                if matrix_with_border[r + loc.0][c + loc.1] == 42 { matrix_of_values[r][c] += 1; }
            }

            // If there are no mines around it, make it a blank space
            if matrix_of_values[r][c] == 0 { matrix_of_values[r][c] = 32; } 
            // Otherwise, add 48 to make it an ASCII character for the number
            else { matrix_of_values[r][c] += 48; } 
        }
    }

    for row in matrix_of_values {
        minemap.push(str::from_utf8(&row).unwrap().to_string());
    }
    minemap
}