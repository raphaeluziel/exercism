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
    frame: u16,
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

        println!("{:?}\n", self);



        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        todo!("HEY")
    }
}
