use std::ops::Range;

const VALID_POSITIONS: Range<i32> = 0..8;
#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if VALID_POSITIONS.contains(&rank) && VALID_POSITIONS.contains(&file) {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || (self.position.rank - other.position.rank).abs()
                == (self.position.file - other.position.file).abs()
    }
}
