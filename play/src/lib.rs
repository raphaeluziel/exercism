pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 { return PascalsTriangle { triangle: Vec::new() };}
        let mut v = Vec::with_capacity(row_count as usize);
        for row in 1..=row_count {
            v.push(vec![1; row as usize]);
        }
        println!("v = {:?}", v);
        PascalsTriangle { 
            triangle: v 
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
