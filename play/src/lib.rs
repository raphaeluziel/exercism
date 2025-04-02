#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

//REmove the derive debug
#[derive(Debug)]
pub struct BowlingGame {
    half_frame: u8,
    pins: Vec<u16>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            half_frame: 0,
            pins: Vec::with_capacity(21)
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft); }

        if self.half_frame % 2 != 0 && pins + self.pins.last().unwrap() > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.half_frame += if pins < 10 { 1 } else { 2 };
        if self.half_frame > 20 { return Err(Error::GameComplete); }

        self.pins.push(pins);
        //self.score += pins;

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        println!("WHAT's Going on, raphael?");
        // 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 7

        if self.half_frame < 10 { return None; }

        let mut sc = 0;
        let mut i = 0;
        println!("WHAT");

        while i < (self.pins.len() - 1) {
            println!("{} = {}, len = {}", i, self.pins[i], self.pins.len());

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

        for j in i..self.pins.len() {
            println!("HEY = {j}");
        }

        // let sc = self.pins.windows(3
        //                       .inspect(|x| println!("{:?}", x))
        //                       .fold(0, |acc, x| acc + x[0]);
        
        //println!("SC = {:?}", sc);
        println!("FRAMES = {}", self.half_frame);

        if self.half_frame < 20 { None } else { Some(sc) }
    }
}
