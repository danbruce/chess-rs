use std::fmt;
use std::ops::Index;

use super::piece::*;

pub struct Tile {
    file: u8,
    rank: u8,
    piece: Option<PieceType>,
}

impl Tile {
    fn from_empty(file: u8, rank: u8) -> Self {
        Tile {
            file,
            rank,
            piece: None,
        }
    }
    fn from_piece(file: u8, rank: u8, piece: PieceType) -> Self {
        Tile {
            file,
            rank,
            piece: Some(piece),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.piece {
            Some(ref piece) => write!(f, "{}", piece.get_piece_letter()),
            None => write!(f, "."),
        }
    }
}

pub struct ChessBoardRank([Tile; 8]);

impl Index<usize> for ChessBoardRank {
    type Output = Tile;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ChessBoardRank {
    fn new_empty(file: u8) -> Self {
        ChessBoardRank([
            Tile::from_empty(0, file),
            Tile::from_empty(1, file),
            Tile::from_empty(2, file),
            Tile::from_empty(3, file),
            Tile::from_empty(4, file),
            Tile::from_empty(5, file),
            Tile::from_empty(6, file),
            Tile::from_empty(7, file),
        ])
    }
    fn new_with_piece(file: u8, piece: PieceType) -> Self {
        ChessBoardRank([
            Tile::from_piece(0, file, piece),
            Tile::from_piece(1, file, piece),
            Tile::from_piece(2, file, piece),
            Tile::from_piece(3, file, piece),
            Tile::from_piece(4, file, piece),
            Tile::from_piece(5, file, piece),
            Tile::from_piece(6, file, piece),
            Tile::from_piece(7, file, piece),
        ])
    }
}

impl fmt::Display for ChessBoardRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self[0])?;
        for col in 1..8 {
            write!(f, " {}", self[col])?;
        }
        Ok(())
    }
}

pub struct ChessBoard([ChessBoardRank; 8]);

impl Index<usize> for ChessBoard {
    type Output = ChessBoardRank;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ChessBoard {
    pub fn new_empty_board() -> Self {
        ChessBoard([
            ChessBoardRank::new_empty(0),
            ChessBoardRank::new_empty(1),
            ChessBoardRank::new_empty(2),
            ChessBoardRank::new_empty(3),
            ChessBoardRank::new_empty(4),
            ChessBoardRank::new_empty(5),
            ChessBoardRank::new_empty(6),
            ChessBoardRank::new_empty(7),
        ])
    }

    pub fn from_start_pos() -> Self {
        let black_rook = PieceType::Rook(Piece::new(PieceColor::Black));
        let black_knight = PieceType::Knight(Piece::new(PieceColor::Black));
        let black_bishop = PieceType::Bishop(Piece::new(PieceColor::Black));
        let black_queen = PieceType::Queen(Piece::new(PieceColor::Black));
        let black_king = PieceType::King(Piece::new(PieceColor::Black));
        let black_pawn = PieceType::Pawn(Piece::new(PieceColor::Black));
        let white_rook = PieceType::Rook(Piece::new(PieceColor::White));
        let white_knight = PieceType::Knight(Piece::new(PieceColor::White));
        let white_bishop = PieceType::Bishop(Piece::new(PieceColor::White));
        let white_queen = PieceType::Queen(Piece::new(PieceColor::White));
        let white_king = PieceType::King(Piece::new(PieceColor::White));
        let white_pawn = PieceType::Pawn(Piece::new(PieceColor::White));
        let board = [
            ChessBoardRank([
                Tile::from_piece(0, 0, black_rook),
                Tile::from_piece(1, 0, black_knight),
                Tile::from_piece(2, 0, black_bishop),
                Tile::from_piece(3, 0, black_queen),
                Tile::from_piece(4, 0, black_king),
                Tile::from_piece(5, 0, black_bishop),
                Tile::from_piece(6, 0, black_knight),
                Tile::from_piece(7, 0, black_rook),
            ]),
            ChessBoardRank::new_with_piece(1, black_pawn),
            ChessBoardRank::new_empty(2),
            ChessBoardRank::new_empty(3),
            ChessBoardRank::new_empty(4),
            ChessBoardRank::new_empty(5),
            ChessBoardRank::new_with_piece(6, white_pawn),
            ChessBoardRank([
                Tile::from_piece(0, 7, white_rook),
                Tile::from_piece(1, 7, white_knight),
                Tile::from_piece(2, 7, white_bishop),
                Tile::from_piece(3, 7, white_queen),
                Tile::from_piece(4, 7, white_king),
                Tile::from_piece(5, 7, white_bishop),
                Tile::from_piece(6, 7, white_knight),
                Tile::from_piece(7, 7, white_rook),
            ]),
        ];
        ChessBoard(board)
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8 {
            write!(f, "{}\n", self[row]).unwrap();
        }
        Ok(())
    }
}
