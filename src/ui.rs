
use std::io::stdin;

use chess::{Square, Piece, Color};
use crate::game::Game;


pub struct ChessUI {
    game: Game,
}

impl ChessUI {

    pub fn from(game : Game) -> ChessUI {
        ChessUI { game }
    }

    fn get_piece_char(piece : Piece, color : Color) -> char {
        let mut unicode_val: u32 = 0x2654;

        match piece {
            Piece::King => unicode_val += 0,
            Piece::Queen => unicode_val += 1,
            Piece::Rook => unicode_val += 2,
            Piece::Bishop => unicode_val += 3,
            Piece::Knight => unicode_val += 4,
            Piece::Pawn => unicode_val += 5
        }

        match color {
            Color::White => unicode_val += 6,
            Color::Black => unicode_val += 0
        }

        return char::from_u32(unicode_val).unwrap();
    }

    pub fn print_board(&self) {
        
        let board = self.game.get_board();
        let mut board_str = String::from("");

        for rank in Game::RANKS {
            for file in Game::FILES { 
                let square = Square::make_square(rank, file);
                let piece = board.piece_on(square);
                let color = board.color_on(square);
                match piece {
                    Some(p) => { board_str.push(Self::get_piece_char(p, color.unwrap()));
                                        board_str.push(' ');
                    },
                    None => board_str.push_str("  ")
                }
            }
            board_str.push('\n');
        }

        println!("{}", board_str);
    }

    pub fn print_legal_moves(&self) {
        
        for legal_move in self.game.get_legal_moves() {
            println!("{}", legal_move);
        }

        println!("{:?}", self.game.get_side_to_move());

    }

    pub fn input_move(&mut self) {

        let mut str_move = String::new();

        while ! self.game.is_move_legal(&str_move) {
            stdin().read_line(&mut str_move).expect("Failed to read line");
            str_move = String::from(str_move.trim());
            if ! self.game.is_move_legal(&str_move) {
                println!("Not valid move");
            }

        }

        self.game.make_move(str_move);

    }

}
