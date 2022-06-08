use std::error::Error;

use regex::Regex;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Piece {
    pieceType: PieceType,
    color: Color,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Color {
    White,
    Black
}

struct Location {
    x: i8,
    y: i8
}

impl Location {
    fn new(x: i8, y: i8) -> Self {
        Location{x, y}
    }
}

struct Game {
    board: ArrayBoard,
}

impl Game {
    fn new() -> Game {
        return Game{board: Board::new()}
    }

    fn get_possible_moves(p: Player) {

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

struct ArrayBoard {
    pieces: [[Option<Piece>; 8]; 8]
}

trait Board {
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
    use std::error::Error;

    use crate::{Game, Piece, Board, Location, ArrayBoard};


    #[test]
    fn fen_tests() -> Result<(), Box<dyn Error>> {
        let input = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
        let game = Game::from_fen(input)?;
        let output = game.to_fen();

        assert_eq!(input, output);
        return Ok(())
    }

    #[test]
    fn place_test() {
        let mut board = ArrayBoard::new();
        let piece = Piece{pieceType: crate::PieceType::King, color: crate::Color::Black};
        board.put(Location{x: 3, y: 4}, Some(piece));
        assert_eq!(Some(piece), board.at(Location{x: 3, y: 4}));
        board.put(Location{x: 3, y: 4}, None);
        assert_eq!(None, board.at(Location{x: 3, y: 4}));
    }

}