#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    bonus: u16,
    last: u16,
    roll: u16,
    frame: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { 
            score: 0,
            bonus: 0,
            last: 0,
            roll: 0,
            frame: 1,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft); 
        }
        if self.frame > 10 || (self.frame == 10 && self.bonus == 0 && self.roll > 1){
            return Err(Error::GameComplete);
        }

        self.roll += 1;
        self.score += pins;
        self.last = pins;

        if pins == 10 { self.bonus += 2; }

        if self.bonus > 0 {
            self.score += pins;
            self.bonus -= 1;
        }

        if self.roll == 1 {
            if pins == 10 { 
                if self.frame != 10 { 
                    self.frame += 1; 
                }
            }
        }
        else if self.roll == 2 {
            if pins + self.last > 10 { return Err(Error::NotEnoughPinsLeft); }
            if pins + self.last == 10 { self.bonus += 1; }
            if self.frame != 10 { 
                self.frame += 1;
                self.roll = 0;
            }
        }
        else {
            self.frame += 1;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame < 10 || 
           (self.frame == 10 && self.roll < 2) || 
           (self.frame == 10 && self.roll == 2 && self.bonus != 0)
            {
                None 
            }
        else {
            Some(self.score) 
        }
    }
}
