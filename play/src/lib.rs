#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    bonus: u16,
    last_roll: u16,
    rolled: u16,
    frame: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { 
            score: 0,
            bonus: 0,
            last_roll: 0,
            rolled: 0,
            frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft); 
        }
        if self.frame > 10 {
            return Err(Error::GameComplete);
        }
        if self.bonus > 0 && self.frame != 10 {
            self.score += pins;
            self.bonus -= 1;
        }
        if pins == 10 {
            self.score += 10 + self.last_roll;
            self.last_roll = 10;
            self.bonus += 2;
            self.frame += 1;
            self.rolled = if self.frame != 10 { 0 } else { self.rolled + 1 }
        }
        else {
            if self.rolled == 0 {
                self.score += pins;
                self.last_roll = pins;
                self.rolled = 1;
            }
            else if self.rolled == 1 {
                let frame_total = pins + self.last_roll;
                if frame_total > 10 { return Err(Error::NotEnoughPinsLeft); }
                if frame_total == 10 { self.bonus += 1; }
                self.score += pins;
                self.last_roll = 0;
                self.frame += 1;
                self.rolled = if self.frame != 10 { 0 } else { 2 }
            }
            else {

            }
        }
        
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame < 10 {
            None 
        } 
        else {
            Some(self.score) 
        }
    }
}
