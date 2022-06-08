use crate::game::{Piece, Location};

pub struct ArrayBoard {
    pieces: [[Option<Piece>; 8]; 8]
}

pub trait Board {
    fn new() -> Self;
    fn at(&mut self, l: Location) -> Option<Piece>;
    fn put(&mut self, l: Location, p: Option<Piece>);
}

impl Board for ArrayBoard {
    fn new() -> ArrayBoard {
        return ArrayBoard{pieces: [[None; 8]; 8]}
    }
    fn at(&mut self, l: Location) -> Option<Piece> {
        return self.pieces[l.x as usize][l.y as usize];
    }

    fn put(&mut self, l: Location, p: Option<Piece>) {
        self.pieces[l.x as usize][l.y as usize] = p;
    }
}

#[cfg(test)] 
mod tests {
    use crate::{board::{ArrayBoard, Board}, game::{Piece, Location, PieceType, Color}};

    #[test]
    fn place_test() {
        let mut board = ArrayBoard::new();
        let piece = Piece{pieceType: PieceType::King, color: Color::Black};
        board.put(Location{x: 3, y: 4}, Some(piece));
        assert_eq!(Some(piece), board.at(Location{x: 3, y: 4}));
        board.put(Location{x: 3, y: 4}, None);
        assert_eq!(None, board.at(Location{x: 3, y: 4}));
    }
}