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
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frame = Frame { score: 0, bonuses: 0, rolls: 0 };
        BowlingGame { 
            frame: 1,
            frames: vec![frame; 10],
        }
    }

    fn game_done(&self) -> bool {

        if self.frame < 10 || 
            self.frames[self.frame - 1].rolls < 2 
            { return false; }
        
        if self.frames[self.frame - 1].rolls < 3 &&
            self.frames[self.frame - 1].bonuses > 0 
            { return false; }

        true
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft); }
        if self.game_done() { return  Err(Error::GameComplete); }

        self.frames[self.frame - 1].score += pins;
        self.frames[self.frame - 1].rolls += 1;

        if self.frame != 10 && self.frames[self.frame - 1].score > 10 { 
            return Err(Error::NotEnoughPinsLeft);
        }
        

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

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_done() { return None; }
        Some(self.frames.iter().map(|x| x.score).sum())
    }
}
