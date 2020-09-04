use Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    marks: Vec<u16>,
    second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            marks: Vec::with_capacity(21),
            second: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second && pins + self.marks.last().unwrap() > 10) {
            Err(NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(GameComplete)
        } else {
            self.marks.push(pins);
            self.second = if pins != 10 { !self.second } else { false };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut score = 0;
        let mut throw = 0;
        let marks = &self.marks;
        for _ in 0..10 {
            let (&first, &second) = (marks.get(throw)?, marks.get(throw + 1)?);
            score += first + second;
            if first == 10 || first + second == 10 {
                score += marks.get(throw + 2)?;
            }
            throw += if first == 10 { 1 } else { 2 };
        }
        Some(score)
    }
}
