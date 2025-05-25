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
        let delta_row = (self.position.row - other.position.row).abs();
        let delta_col = (self.position.col - other.position.col).abs();

        delta_row == 0 || delta_col == 0 || delta_row == delta_col
    }
}