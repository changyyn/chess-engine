use std::str::FromStr;

use crate::player::{Player, HumanPlayer, BotPlayer};

use chess::{Board, Rank, File, ChessMove, MoveGen, Color, BoardStatus};

pub struct Game {
    player_white: Box<dyn Player>,
    player_black: Box<dyn Player>,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            player_white: Box::new(HumanPlayer::default()),
            player_black: Box::new(BotPlayer::default()),
            board: Board::default(),
        }
    }
}

impl Game {

    // pub const RANKS : [Rank; 8] = [Rank::First, Rank::Second, Rank::Third, Rank::Fourth, Rank::Fifth, Rank::Sixth, Rank::Seventh, Rank::Eighth];
    pub const RANKS : [Rank; 8] = [Rank::Eighth, Rank::Seventh, Rank::Sixth, Rank::Fifth, Rank::Fourth, Rank::Third, Rank::Second, Rank::First];
    pub const FILES : [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];
    
    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    pub fn get_status(&self) -> BoardStatus {
        self.board.status()
    } 

    pub fn get_legal_moves(&self) -> MoveGen {
        MoveGen::new_legal(&self.board)
    }

    pub fn get_side_to_move(&self) -> Color {
        self.board.side_to_move()
    }

    pub fn take_current_player_turn(&mut self) {
        let chess_move = match self.get_side_to_move() {
            Color::White => self.player_white.make_move(&self.board),
            Color::Black => self.player_black.make_move(&self.board)
        };

        self.make_move(chess_move.to_string());
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

