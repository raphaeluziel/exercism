#[derive(Debug)]
pub struct HighScores{
    scrs: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scrs: Vec::from(scores) }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scrs
    }

    pub fn latest(&self) -> Option<u32> {
        self.scrs.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        println!("HEY {:?}", self.scrs);
        let mut x = [];
        x.clone_from_slice(&self.scrs);
        
        println!("XXX {:?}", x);
        todo!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        todo!("Return 3 highest scores")
    }
}