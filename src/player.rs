use chess::{Board, Color, ChessMove};

use crate::util::{human_input_functions, eval_functions};

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
    eval_fn : fn(&Board) -> ChessMove,
}

impl Default for BotPlayer {
    fn default() -> Self {
        BotPlayer { color: Color::Black, eval_fn: eval_functions::random_eval }
    }
}

impl Player for BotPlayer {
    fn get_color(&self) -> Color {
        self.color
    }

    fn make_move(&self, board : &Board) -> ChessMove {
        (self.eval_fn)(&board)
    }
}

