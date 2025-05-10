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

        // let mut v = vec![1; 4];
        // v.insert(1, triangle[4][0] + triangle[4][1]);
        // v.insert(2, triangle[4][1] + triangle[4][2]);

        let mut v:Vec<u32> = Vec::new();
        v.push(1);
        v.push(triangle[4][0] + triangle[4][1]);
        v.push(triangle[4][1] + triangle[4][2]);
        v.push(triangle[4][1] + triangle[4][2]);
        v.push(triangle[4][0] + triangle[4][1]);
        v.push(1);

        let mut ggg = vec![1u32];
        ggg.extend(triangle[4].windows(2).map(|x| x[0] + x[1]));
        ggg.push(1);
        println!("GGG = {:?}", ggg);

        triangle.push(v.clone());

        let mut w:Vec<u32> = Vec::new();
        w.push(1);
        w.push(triangle[5][0] + triangle[5][1]);
        w.push(triangle[5][1] + triangle[5][2]);
        w.push(triangle[5][2] + triangle[5][3]);
        w.push(triangle[5][1] + triangle[5][2]);
        w.push(triangle[5][0] + triangle[5][1]);
        w.push(1);

        println!("Sizes = {} and {}", triangle[4].len()/2, v.len()/2);
        println!("Row 4 = {:?}", triangle[4]);
        println!("Row 5 = [1, 5, 10, 10, 5, 1]");
        println!("Row 5 = {:?}", v);

        println!("W = {:?}", w);
        //PascalsTriangle { triangle }
        todo!()
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
