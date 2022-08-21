pub mod human_input_functions {
    use std::str::FromStr;
    use std::io::stdin;

    use chess::{Board, ChessMove};
 
    pub fn console_input(board : &Board) -> ChessMove {

        loop {
            let mut str_move = String::new();
            stdin().read_line(&mut str_move).expect("Failed to read line");
            str_move = String::from(str_move.trim());
            let chess_move_res = ChessMove::from_str(&str_move);

            match chess_move_res {
                Ok(chess_move) => {
                    if board.legal(chess_move) {
                        return chess_move
                    }
                    println!("Not a legal move");
                },
                Err(_) => {
                    println!("String input does not form valid move");
                }
            }
        }
    
    }
}

pub mod eval_functions {
    use rand::{self, seq::IteratorRandom};

    use chess::{Board, MoveGen, ChessMove};


    pub fn random_eval(board : &Board) -> ChessMove  {
        let chess_moves = MoveGen::new_legal(&board);
        
        chess_moves.choose(&mut rand::thread_rng()).unwrap()
    }


}