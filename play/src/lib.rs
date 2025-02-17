#[derive(Debug)]
pub struct HighScores{
    scrs: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        println!("SCORES = {:?}", scores);
        println!("HIGHSCORES = {:?}", HighScores { scrs: Vec::from(scores) });
        HighScores { scrs: Vec::from(scores) }
    }

    pub fn scores(&self) -> &[u32] {
        println!("SELF {:?}", &self.scrs);
        //todo!("Return all the scores as a slice")
        &self.scrs
    }

    pub fn latest(&self) -> Option<u32> {
        todo!("Return the latest (last) score")
    }

    pub fn personal_best(&self) -> Option<u32> {
        todo!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        todo!("Return 3 highest scores")
    }
}