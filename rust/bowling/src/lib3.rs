#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone)]
pub struct Frame {
    score: u16,
    bonuses: u16,
    rolls: u16,
}

pub struct BowlingGame {
    frame: usize,
    frames: Vec<Frame>,
    pins_left: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frame = Frame {
            score: 0,
            bonuses: 0,
            rolls: 0,
        };
        BowlingGame {
            frame: 1,
            frames: vec![frame; 10],
            pins_left: 10,
        }
    }

    fn game_done(&self) -> bool {
        let i = self.frame - 1;
        !(self.frame < 10
            || self.frames[i].rolls < 2
            || self.frames[i].rolls < 3 
            && self.frames[i].bonuses > 0)
    }

    fn add_bonuses(&mut self, i: usize, pins: u16) {
        if self.frames[i].bonuses == 0 { return; }
        self.frames[i].score += pins;
        self.frames[i].bonuses -= 1;
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.pins_left { return Err(Error::NotEnoughPinsLeft); }
        if self.game_done() { return Err(Error::GameComplete); }

        if pins == 10 { self.pins_left = 10; } 
        else { self.pins_left -= pins; }

        let i = self.frame - 1;
        self.frames[i].score += pins;
        self.frames[i].rolls += 1;

        for j in 1..3 {
            if self.frame > j {
                self.add_bonuses(self.frame - (j + 1), pins);
            }
        }

        if self.frames[i].score < 10
            && self.frame != 10
            && self.frames[i].rolls == 2 {
                self.frame += 1;
                self.pins_left = 10;
            } 
        else if self.frames[i].score == 10 {
            self.frames[i].bonuses = if pins == 10 { 2 } else { 1 };
            self.pins_left = 10;

            if self.frame != 10 { self.frame += 1; }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_done() { return None; }
        Some(self.frames.iter().map(|x| x.score).sum())
    }
}
