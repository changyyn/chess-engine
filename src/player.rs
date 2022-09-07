use chess::{Board, ChessMove, MoveGen};

pub enum Player {
    Human(fn(&Board)->ChessMove),
    Bot(fn(&Board)->f64),
}

impl Player {
    pub fn make_move(&self, board : &Board) -> ChessMove{
        use rand::{seq::IteratorRandom};
        
        match *self { 
            Player::Human(f) => f(board),
            Player::Bot(_) => {
                let chess_moves = MoveGen::new_legal(&board);
        
                chess_moves.choose(&mut rand::thread_rng()).unwrap()
            }
        }
    }
}

