use std::str::FromStr;

use chess::{Board, Rank, File, ChessMove, MoveGen, Color};

pub struct Game {
   board: Board,
}

impl Game {

    // pub const RANKS : [Rank; 8] = [Rank::First, Rank::Second, Rank::Third, Rank::Fourth, Rank::Fifth, Rank::Sixth, Rank::Seventh, Rank::Eighth];
    pub const RANKS : [Rank; 8] = [Rank::Eighth, Rank::Seventh, Rank::Sixth, Rank::Fifth, Rank::Fourth, Rank::Third, Rank::Second, Rank::First];
    pub const FILES : [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];

    pub fn default() -> Game {
        return Game { board: (Board::default())}
    }

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    pub fn get_legal_moves(&self) -> MoveGen {
        MoveGen::new_legal(&self.board)
    }

    pub fn get_side_to_move(&self) -> Color {
        self.board.side_to_move()
    }

    pub fn is_move_legal(&self, move_str : &String) -> bool {
        
        if move_str.len() != 4 {
            return false;
        }

        return self.board.legal(ChessMove::from_str(move_str).unwrap());
    }

    pub fn make_move(&mut self, move_str: String) {
        let chess_move_result = ChessMove::from_str(&move_str[..]);
        let mut chess_move = ChessMove::default();

        match chess_move_result {
            Ok(m) => chess_move = m,
            Err(_) => println!("ERROR MOVE NOT PROCESSED")
        }

        self.board = self.board.make_move_new(chess_move);
    }


}

