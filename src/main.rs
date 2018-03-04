use std::fmt;

const BOARD_DIMENSION: (usize, usize) = (8, 8);

#[derive(Clone, Copy)]
struct Piece {
    letter: char,
}

#[derive(Clone, Copy)]
struct PossiblePiece(Option<Piece>);

impl fmt::Display for PossiblePiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(piece) => write!(f, "{}", piece.letter),
            None => write!(f, "."),
        }
    }
}

trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for Option<Piece> {
    fn to_char(&self) -> char {
        match *self {
            Some(p) => p.letter,
            None => '.',
        }
    }
}

struct ChessBoard {
    board: [[PossiblePiece; BOARD_DIMENSION.0]; BOARD_DIMENSION.1],
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..(BOARD_DIMENSION.1 - 1) {
            write!(f, "{}", self.board[row][0])?;
            for col in 1..BOARD_DIMENSION.0 {
                write!(f, " {}", self.board[row][col])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "{}", self.board[BOARD_DIMENSION.1 - 1][0])?;
        for col in 1..BOARD_DIMENSION.0 {
            write!(f, " {}", self.board[BOARD_DIMENSION.1 - 1][col])?;
        }
        Ok(())
    }
}

impl ChessBoard {
    fn from_empty_board() -> Self {
        let board = [[PossiblePiece(None); BOARD_DIMENSION.0]; BOARD_DIMENSION.1];
        ChessBoard { board }
    }

    fn from_start_pos() -> Self {
        let r = Piece { letter: 'R' };
        let n = Piece { letter: 'N' };
        let b = Piece { letter: 'B' };
        let q = Piece { letter: 'Q' };
        let k = Piece { letter: 'K' };
        let p = Piece { letter: 'p' };
        let board = [
            [
                PossiblePiece(Some(r)),
                PossiblePiece(Some(n)),
                PossiblePiece(Some(b)),
                PossiblePiece(Some(q)),
                PossiblePiece(Some(k)),
                PossiblePiece(Some(b)),
                PossiblePiece(Some(n)),
                PossiblePiece(Some(r)),
            ],
            [PossiblePiece(Some(p)); BOARD_DIMENSION.0],
            [PossiblePiece(None); BOARD_DIMENSION.0],
            [PossiblePiece(None); BOARD_DIMENSION.0],
            [PossiblePiece(None); BOARD_DIMENSION.0],
            [PossiblePiece(None); BOARD_DIMENSION.0],
            [PossiblePiece(Some(p)); BOARD_DIMENSION.0],
            [
                PossiblePiece(Some(r)),
                PossiblePiece(Some(n)),
                PossiblePiece(Some(b)),
                PossiblePiece(Some(q)),
                PossiblePiece(Some(k)),
                PossiblePiece(Some(b)),
                PossiblePiece(Some(n)),
                PossiblePiece(Some(r)),
            ],
        ];
        ChessBoard { board }
    }
}

fn main() {
    let board = ChessBoard::from_empty_board();
    println!("{}", board);
    println!("");
    let initial_board = ChessBoard::from_start_pos();
    println!("{}", initial_board);
}
