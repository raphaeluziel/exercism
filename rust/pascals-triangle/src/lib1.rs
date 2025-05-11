pub struct PascalsTriangle {
    rows: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {

        if self.rows == 0 { return Vec::new(); }

        let mut triangle:Vec<Vec<u32>> = Vec::with_capacity(self.rows as usize);

        triangle.push(vec![1]);
        
        for row in 0..(self.rows - 1) {
            let mut v = vec![1];
            v.extend(triangle[row as usize].windows(2).map(|x| x[0] + x[1]));
            v.push(1);
            triangle.push(v);
        }

        triangle
    }
}
