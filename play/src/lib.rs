pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut plants:Vec<&str> = Vec::new();
    let i = 2 * (student.as_bytes().first().unwrap_or(&65) - 65) as usize;
    let second_row = diagram.len() / 2 + 1;
    let positions = [i, (i + 1), (i + second_row), (i + second_row + 1)];

    for p in positions {
        plants.push(match diagram.as_bytes()[p] {
            67 => "clover",
            71 => "grass",
            82 => "radishes",
            86 => "violets",
            _ => ""
        })
    }

    plants
}