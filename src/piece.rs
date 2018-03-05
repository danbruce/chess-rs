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
        match self {
            &PieceType::King(ref k) => match k.color {
                PieceColor::White => 'K',
                PieceColor::Black => 'k',
            },
            &PieceType::Queen(ref q) => match q.color {
                PieceColor::White => 'Q',
                PieceColor::Black => 'q',
            },
            &PieceType::Bishop(ref b) => match b.color {
                PieceColor::White => 'B',
                PieceColor::Black => 'b',
            },
            &PieceType::Knight(ref n) => match n.color {
                PieceColor::White => 'N',
                PieceColor::Black => 'n',
            },
            &PieceType::Rook(ref r) => match r.color {
                PieceColor::White => 'R',
                PieceColor::Black => 'r',
            },
            &PieceType::Pawn(ref p) => match p.color {
                PieceColor::White => 'P',
                PieceColor::Black => 'p',
            },
        }
    }
}
