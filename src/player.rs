use std::str::FromStr;

use chess::{Board, Color, ChessMove};

pub trait Player {
    fn get_color(&self) -> Color;
    fn make_move(&self, board : &Board) -> ChessMove ;
}

pub struct HumanPlayer {
    color : Color,
}

impl Player for HumanPlayer {

    fn get_color(&self) -> Color {
        self.color
    }

    fn make_move(&self, board : &Board) -> ChessMove {
        ChessMove::from_str("g1h3").unwrap()
    }
}

impl HumanPlayer {
    pub fn default() -> HumanPlayer {
        HumanPlayer { color: Color::Black }
    }
}

type EvalFn = fn(&Color, &Board) -> f32;
pub struct BotPlayer {
    color : Color,
}

impl Player for BotPlayer {
    fn get_color(&self) -> Color {
        self.color
    }

    fn make_move(&self, board : &Board) -> ChessMove {
        return ChessMove::from_str("g1h3").unwrap();
    }
}

impl BotPlayer {
    pub fn default() -> BotPlayer {
        BotPlayer { color: Color::Black }
    }
}