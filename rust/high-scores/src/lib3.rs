// Third iteration based on auto suggestions given

#[derive(Debug)]
pub struct HighScores<'a> {
    scrs: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scrs: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scrs
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