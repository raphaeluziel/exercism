#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

//REmove the derive debug
#[derive(Debug)]
pub struct BowlingGame {
    score: u16,
    half_frame: u8,
    pins: Vec<u16>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            score: 0,
            half_frame: 0,
            pins: Vec::with_capacity(21)
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft); }

        self.half_frame += 1;

        self.pins.push(pins);
        //self.score += pins;

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // 6, 2, 10, 3, 5, 2, 2, 10, 10, 7, 3, 1, 1, 6, 4, 7, 3, 2

        if self.pins.len() < 20 { return None; }

        let mut sc = 0;
        let mut i = 0;

        while i < (self.pins.len() - 2) {
            println!("{}", self.pins[i]);

            let fr = self.pins[i] + self.pins[i+1];
            if fr < 10 { 
                sc += fr;
                i += 2;
            }
            else if fr == 10 { 
                sc += fr + self.pins[i+2];
                i += 2
            }
            else if self.pins[i] == 10 {
                sc += 10 + self.pins[i+1] + self.pins[i+2];
                i += 1;
            }
        }

        // let sc = self.pins.windows(3
        //                       .inspect(|x| println!("{:?}", x))
        //                       .fold(0, |acc, x| acc + x[0]);
        
        println!("SC = {:?}", sc);

        if self.half_frame < 20 { None } else { Some(sc) }
    }
}
