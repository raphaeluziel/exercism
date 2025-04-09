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
    frame: u16,
    fill_balls: u16
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { 
            score: 0, 
            bonus: 0,
            last_roll: 0,
            frame_start: true,
            frame: 0,
            fill_balls: 0
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        println!("FILL = {}", self.fill_balls);
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft); 
        }
        if self.frame > 10 || (self.frame == 10 && self.frame_start && self.fill_balls == 0) {
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
            self.frame_start = true;
            if self.frame == 10 { self.fill_balls = 2; }
        }
        else {
            if self.frame_start {
                self.score += pins;
                self.last_roll = pins;
                self.frame_start = false;
                self.fill_balls = 0;
            }
            else {
                let frame_total = pins + self.last_roll;
                if frame_total > 10 { return Err(Error::NotEnoughPinsLeft); }
                if frame_total == 10 { self.bonus += 1; }
                self.score += pins;
                self.last_roll = 0;
                self.frame += 1;
                self.frame_start = true;
                if self.frame == 10 { self.fill_balls = 1; }
            }
        }
        
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame < 10 || (self.frame == 10 && self.fill_balls > 0 && !self.frame_start) {
            None 
        } 
        else {
            Some(self.score) 
        }
    }
}
