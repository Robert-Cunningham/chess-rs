use regex::Regex;

fn main() {
    println!("Hello, world!");
}

// ultimately exposes things like get white pieces, get white possible moves.


struct Move {}


enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
}

struct Piece {
    pieceType: PieceType,
    color: Color,
}

enum Color {
    White,
    Black
}

enum Player {
    White,
    Black
}

struct Location {
    x: i8,
    y: i8
}

struct Game {
}

impl Game {
    fn new() -> Game {
        return Game{}
    }

    fn get_possible_moves(p: Player) {

    }


    // 8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50
    fn from_fen(s: &str) -> Game {
        let re = Regex::new(r"(.*/) (?P<turn>[wb]) .* .* .* .*");
        let g = Game::new();
        return g;
    }

    fn to_fen(&self) -> &str {
        return ""
    }
}

struct Board {
    pieces: [i8; 64]
}

impl Board {
    fn at(l: Location) -> Piece {
        Piece{pieceType: PieceType::Pawn, color: Color::Black}
    }

    fn put(l: Location, p: Piece) {

    }
}

#[cfg(test)]
mod tests {
    use crate::Game;


    #[test]
    fn fen_tests() {
        let input = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
        let game = Game::from_fen(input);
        let output = game.to_fen();

        assert_eq!(input, output)
    }

}