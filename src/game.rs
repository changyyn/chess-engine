use std::str::FromStr;

use chess::{Board, Rank, File, ChessMove};

pub struct Game {
   board: Board,
}

impl Game {

    pub const RANKS : [Rank; 8] = [Rank::First, Rank::Second, Rank::Third, Rank::Fourth, Rank::Fifth, Rank::Sixth, Rank::Seventh, Rank::Eighth];
    pub const FILES : [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];

    pub fn default() -> Game {
        return Game { board: (Board::default())}
    }

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    pub fn is_move_legal(&self, move_str : &String) -> bool {
        
        if move_str.len() != 4 {
            return false;
        }

        return self.board.legal(ChessMove::from_str(move_str).unwrap());
    }


}

