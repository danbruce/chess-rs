use std::fmt;

#[derive(Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
}

impl Piece {
    pub fn new(color: PieceColor) -> Self {
        Piece { color }
    }
}

#[derive(Clone, Copy)]
pub enum PieceType {
    King(Piece),
    Queen(Piece),
    Bishop(Piece),
    Knight(Piece),
    Rook(Piece),
    Pawn(Piece),
}

impl PieceType {
    pub fn get_piece_letter(&self) -> char {
        match *self {
            PieceType::King(ref k) => match k.color {
                PieceColor::White => 'K',
                PieceColor::Black => 'k',
            },
            PieceType::Queen(ref q) => match q.color {
                PieceColor::White => 'Q',
                PieceColor::Black => 'q',
            },
            PieceType::Bishop(ref b) => match b.color {
                PieceColor::White => 'B',
                PieceColor::Black => 'b',
            },
            PieceType::Knight(ref n) => match n.color {
                PieceColor::White => 'N',
                PieceColor::Black => 'n',
            },
            PieceType::Rook(ref r) => match r.color {
                PieceColor::White => 'R',
                PieceColor::Black => 'r',
            },
            PieceType::Pawn(ref p) => match p.color {
                PieceColor::White => 'P',
                PieceColor::Black => 'p',
            },
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceType::King(ref k) => match k.color {
                PieceColor::White => write!(f, "White King"),
                PieceColor::Black => write!(f, "Black King"),
            },
            PieceType::Queen(ref k) => match k.color {
                PieceColor::White => write!(f, "White Queen"),
                PieceColor::Black => write!(f, "Black Queen"),
            },
            PieceType::Bishop(ref k) => match k.color {
                PieceColor::White => write!(f, "White Bishop"),
                PieceColor::Black => write!(f, "Black Bishop"),
            },
            PieceType::Knight(ref k) => match k.color {
                PieceColor::White => write!(f, "White Knight"),
                PieceColor::Black => write!(f, "Black Knight"),
            },
            PieceType::Rook(ref k) => match k.color {
                PieceColor::White => write!(f, "White Rook"),
                PieceColor::Black => write!(f, "Black Rook"),
            },
            PieceType::Pawn(ref k) => match k.color {
                PieceColor::White => write!(f, "White Pawn"),
                PieceColor::Black => write!(f, "Black Pawn"),
            },
        }
    }
}
