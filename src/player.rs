use std::str::FromStr;

use chess::{Board, Color, ChessMove};

use crate::util::human_input_functions;

pub trait Player {
    fn get_color(&self) -> Color;
    fn make_move(&self, board : &Board) -> ChessMove ;
}

pub struct HumanPlayer {
    color : Color,
    input_fn : fn(&Board) -> ChessMove,
}

impl Default for HumanPlayer {
    fn default() -> Self {
        HumanPlayer {
            color : Color::White,
            input_fn : human_input_functions::console_input,
        }
    }
}

impl Player for HumanPlayer {

    fn get_color(&self) -> Color {
        self.color
    }

    fn make_move(&self, board : &Board) -> ChessMove {
        (self.input_fn)(&board)
    }
}

pub struct BotPlayer {
    color : Color,
}

impl Player for BotPlayer {
    fn get_color(&self) -> Color {
        self.color
    }

    fn make_move(&self, board : &Board) -> ChessMove {
        board.is_sane();
        return ChessMove::from_str("g1h3").unwrap();
    }
}

impl BotPlayer {
    pub fn default() -> BotPlayer {
        BotPlayer { color: Color::Black }
    }
}