use crate::Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    location: Point,
    facing: Direction,
}

struct Point {
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            location: Point { x, y },
            facing: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.facing {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        Self {
            facing: new_direction,
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.facing {
            North => West,
            East => North,
            South => East,
            West => South,
        };

        Self {
            facing: new_direction,
            ..self
        }
    }

    pub fn advance(self) -> Self {
        let new_location = match self.facing {
            North => Point {
                y: self.location.y + 1,
                ..self.location
            },
            East => Point {
                x: self.location.x + 1,
                ..self.location
            },
            South => Point {
                y: self.location.y - 1,
                ..self.location
            },
            West => Point {
                x: self.location.x - 1,
                ..self.location
            },
        };

        Self {
            location: new_location,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'A' => robot.advance(),
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.location.x, self.location.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
