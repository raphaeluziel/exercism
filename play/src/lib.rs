fn move_right(ser: (usize, usize)) -> (usize, usize, bool) {
    (ser.0, ser.1, false)
}

fn move_down(ser: (usize, usize)) -> (usize, usize, bool) {
    (ser.0 + 1, ser.1, false)
}

fn move_left(ser: (usize, usize)) -> (usize, usize, bool) {
    (ser.0 - 1, ser.1 - 1, true)
}

fn move_up(ser: (usize, usize)) -> (usize, usize, bool) {
    (ser.0 + 1, ser.1, true)
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = 7;

    let mut v = vec![vec![0u32; size as usize]; size as usize];

    if size == 0 { return v; }
    if size == 1 { return vec![vec![1]]; }

    let last = size * size;
    let mut count = 1usize;
    let mut start = 0usize;
    let mut end = size;

    while count < last {
        
    }

    todo!()
}