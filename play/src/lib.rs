use std::pin;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    bonus: u16,
    spare: bool,
    strike: bool,
    frame: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { 
            score: 0, 
            bonus: 0, 
            spare: false,
            strike: false,
            frame: Vec::with_capacity(10)
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if pins == 10 {
            self.strike = true;
            self.frame.push(pins);
        }
        
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {

        // if self.half_frames < 10 { return None; }

        // let mut sc = 0;
        // let mut i = 0;

        // while i < (self.pins.len() - 1) {

        //     let fr = self.pins[i] + self.pins[i+1];
        //     if fr < 10 { 
        //         sc += fr;
        //         i += 2;
        //     }
        //     else if fr == 10 { 
        //         sc += fr + self.pins[i+2];
        //         i += 2
        //     }
        //     else if self.pins[i] == 10 {
        //         sc += 10 + self.pins[i+1] + self.pins[i+2];
        //         i += 1;
        //     }
        // }

        // if self.half_frames < 20 { None } else { Some(sc) }
        todo!()
    }
}
