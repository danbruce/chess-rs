mod piece;
mod board;

fn main() {
    let board = board::ChessBoard::new_empty_board();
    print!("{}", board);
    println!("");
    let default_board = board::ChessBoard::from_start_pos();
    print!("{}", default_board);
    println!("");
    for rank in 0..8 {
        for file in 0..8 {
            match default_board.piece_at_pos(rank, file) {
                Some(p) => println!("{}", p),
                None => println!("No piece."),
            };
        }
    }
}
