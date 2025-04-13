use std::pin;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Debug)]
pub struct Frame {
    score: u16,
    bonuses: u16,
    rolls: u16
}

#[derive(Debug)]
pub struct BowlingGame {
    frame: usize,
    frames: Vec<Frame>
}

impl BowlingGame {
    pub fn new() -> Self {
        let frame = Frame { score: 0, bonuses: 0, rolls: 0 };
        BowlingGame { 
            frame: 1,
            frames: vec![frame; 10]
        }
    }


    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft); }

        //print!("Frame = {}, ", self.frame);  

        self.frames[self.frame - 1].score += pins;
        self.frames[self.frame - 1].rolls += 1;

        if self.frame > 1 {
            if self.frames[self.frame - 2].bonuses > 0 {
                self.frames[self.frame - 2].score += pins;
                self.frames[self.frame - 2].bonuses -= 1;
            }
        }
        if self.frame > 2 {
            if self.frames[self.frame - 3].bonuses > 0 {
                self.frames[self.frame - 3].score += pins;
                self.frames[self.frame - 3].bonuses -= 1;
            }
        }

        if self.frames[self.frame - 1].score < 10 {
            if self.frame != 10 && self.frames[self.frame - 1].rolls == 2 { self.frame += 1; }
        }

        else if self.frames[self.frame - 1].score == 10 {
            self.frames[self.frame - 1].bonuses = if pins == 10 { 2 } else { 1 };
            if self.frame != 10 { self.frame += 1; }
        }

        else {
            return Err(Error::NotEnoughPinsLeft);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // if self.frame == 10 && self.frames[9].rolls == 2 || self.frames[9].rolls == 3 && self.frames[9].score == 10 { }
        Some(self.frames.iter().map(|x| x.score).sum())
    }
}
