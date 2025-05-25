#[derive(Debug)]
pub struct ChessPosition {
    row: i32,
    col: i32
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank > 7 || file > 7 { return None; }
        Some(ChessPosition { row: rank, col: file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let r1 = self.position.row;
        let c1 = self.position.col;
        let r2 = other.position.row;
        let c2 = other.position.col;

        r1 == r2 || c1 == c2 || (r1 - r2).abs() == (c1 - c2).abs()
    }
}