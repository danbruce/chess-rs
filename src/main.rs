mod piece;
mod board;

fn main() {
    let board = board::ChessBoard::new_empty_board();
    print!("{}", board);
    println!("");
    let default_board = board::ChessBoard::from_start_pos();
    print!("{}", default_board);
}
