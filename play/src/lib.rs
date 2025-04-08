#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    bonus: u16,
    last_roll: u16,
    frame_start: bool,
    frames: u16
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { 
            score: 0, 
            bonus: 0,
            last_roll: 0,
            frame_start: true,
            frames: 0
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft); 
        }
        if self.frames > 10 || (self.frames == 10 && self.frame_start) {
            return Err(Error::GameComplete);
        }
        if self.bonus > 0 {
            self.score += pins;
            self.bonus -= 1;
        }
        if pins == 10 {
            self.score += 10;
            self.last_roll = 10;
            self.bonus += 2;
            self.frames += 1;
            self.frame_start = true;
        }
        else {
            if self.frame_start {
                self.score += pins;
                self.last_roll = pins;
                self.frame_start = false;
            }
            else {
                let frame_total = pins + self.last_roll;
                if frame_total > 10 { return Err(Error::NotEnoughPinsLeft); }
                if frame_total == 10 { self.bonus += 1; }
                self.score += pins;
                self.last_roll = 0;
                self.frames += 1;
                self.frame_start = true;
            }
        }
        
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames < 10 { return None; }
        Some(self.score)
    }
}
