use std::error::Error;
use regex::Regex;

use crate::board::{ArrayBoard, Board};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub pieceType: PieceType,
    pub color: Color,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black
}

pub struct Location {
    pub x: i8,
    pub y: i8
}

impl Location {
    pub fn new(x: i8, y: i8) -> Self {
        Location{x, y}
    }
}

pub struct Game {
    board: ArrayBoard,
}

impl Game {
    fn new() -> Game {
        return Game{board: Board::new()}
    }

    fn get_possible_moves(c: Color) {

    }

    // 8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50
    fn from_fen(s: &str) -> Result<Game, Box<dyn Error>> {
        let re = Regex::new(r"(.*/) (?P<turn>[wb]) .* .* .* .*")?;
        let mut g = Game::new();
        for m in re.captures_iter(s) {
            println!("{:?}",m)
        }

        g.board.put(Location::new(3, 5), Some(Piece{pieceType: PieceType::Bishop, color: Color::Black}));
        return Ok(g);
    }

    fn to_fen(&self) -> &str {
        return ""
    }
}


#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::game::Game;

    #[test]
    fn fen_tests() -> Result<(), Box<dyn Error>> {
        let input = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
        let game = Game::from_fen(input)?;
        let output = game.to_fen();

        assert_eq!(input, output);
        return Ok(())
    }
}