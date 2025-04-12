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

    fn game_completed(&self) -> bool {
        println!("BBBFrame = {}, Roll = {}, Bonus = {}", self.frame, self.roll, self.bonus);
        if self.frame == 10 &&
           (self.roll == 3 ||
           (self.frame == 10 && (self.roll == 2 && self.bonus == 0)))  { true }
        else { false }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft); }
        if self.game_completed() { return Err(Error::GameComplete); }

        println!("SCORE = {}", self.score);

        self.roll += 1;
        self.score += pins;
        self.last = pins;

        if self.bonus > 0 {
            self.score += pins;
            self.bonus -= 1;
        }

        if self.roll == 1 {
            if pins == 10 { 
                self.bonus += 2;
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
        //println!("---------\nFrame = {}, Roll = {}, Bonus = {}\n---------", self.frame, self.roll, self.bonus);
        if !self.game_completed() { None } else { Some(self.score) }
    }
}
