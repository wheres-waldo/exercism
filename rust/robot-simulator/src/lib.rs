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
        Robot {
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

        Robot {
            location: self.location,
            facing: new_direction,
        }
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.facing {
            North => West,
            East => North,
            South => East,
            West => South,
        };

        Robot {
            location: self.location,
            facing: new_direction,
        }
    }

    pub fn advance(self) -> Self {
        let new_location = match self.facing {
            North => Point {
                x: self.location.x,
                y: self.location.y + 1,
            },
            East => Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
            South => Point {
                x: self.location.x,
                y: self.location.y - 1,
            },
            West => Point {
                x: self.location.x - 1,
                y: self.location.y,
            },
        };

        Robot {
            location: new_location,
            facing: self.facing,
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
