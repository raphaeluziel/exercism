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
        let mut sorted_scores = self.scrs.to_vec();
        sorted_scores.sort();
        sorted_scores.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = self.scrs.to_vec();
        top_three.sort();
        top_three.reverse();
        top_three.iter().enumerate().filter(|x| x.0 < 3).map(|x| *x.1).collect()
    }
}