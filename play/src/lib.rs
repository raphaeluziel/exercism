pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 { return PascalsTriangle { triangle: Vec::new() };}
        let mut triangle = Vec::with_capacity(row_count as usize);

        triangle.push(vec![1]);
        triangle.push(vec![1, 1]);
        triangle.push(vec![1, 2, 1]);
        triangle.push(vec![1, 3, 3, 1]);
        triangle.push(vec![1, 4, 6, 4, 1]);

        let mut v = vec![1; 4];
        v.insert(1, triangle[4][0] + triangle[4][1]);
        v.insert(2, triangle[4][1] + triangle[4][2]);
        

        // let mut v:Vec<u32> = Vec::new();
        // v.push(1);
        // v.push(triangle[4][0] + triangle[4][1]);
        // v.push(triangle[4][1] + triangle[4][2]);
        
        //v.rotate_left(2);
        // v.push(triangle[4][2] + triangle[4][3]);
        // v.push(triangle[4][3] + triangle[4][4]);
        // v.push(1);
        
        println!("Sizes = {} and {}", triangle[4].len()/2, v.len()/2);
        println!("Row 4 = {:?}", triangle[4]);
        println!("Row 5 = [1, 5, 10, 10, 5, 1]");
        println!("Row 5 = {:?}", v);


        PascalsTriangle { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
