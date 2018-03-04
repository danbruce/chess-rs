use std::fmt;

const BOARD_DIMENSION: (usize, usize) = (8, 8);

#[derive(Clone, Copy)]
struct Piece {
    letter: char,
}

struct ChessBoard {
    board: [[Option<Piece>; BOARD_DIMENSION.0]; BOARD_DIMENSION.1],
}

impl fmt::Debug for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..BOARD_DIMENSION.1 {
            f.write_str(&self.board[row]
                .into_iter()
                .map(|p| match *p {
                    Some(piece) => piece.letter.to_string(),
                    None => '.'.to_string(),
                })
                .collect::<Vec<String>>()
                .join(" "))?;
            f.write_str(&"\n".to_string())?;
        }
        Ok(())
    }
}

impl ChessBoard {
    fn from_empty_board() -> Self {
        let board = [[None; BOARD_DIMENSION.0]; BOARD_DIMENSION.1];
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
                Some(r),
                Some(n),
                Some(b),
                Some(q),
                Some(k),
                Some(b),
                Some(n),
                Some(r),
            ],
            [Some(p); BOARD_DIMENSION.0],
            [None; BOARD_DIMENSION.0],
            [None; BOARD_DIMENSION.0],
            [None; BOARD_DIMENSION.0],
            [None; BOARD_DIMENSION.0],
            [Some(p); BOARD_DIMENSION.0],
            [
                Some(r),
                Some(n),
                Some(b),
                Some(q),
                Some(k),
                Some(b),
                Some(n),
                Some(r),
            ],
        ];
        ChessBoard { board }
    }
}

fn main() {
    let board = ChessBoard::from_empty_board();
    println!("{:?}", board);
    println!("");
    let initial_board = ChessBoard::from_start_pos();
    println!("{:?}", initial_board);
}
