use chess::{Board, MoveGen};

fn main() {
    let board = Board::default();
    let movegen = MoveGen::new_legal(&board);

    for m in movegen {
        println!("{}", m);
    }
}
