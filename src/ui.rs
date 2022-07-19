
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
            Color::White => unicode_val += 0,
            Color::Black => unicode_val += 6
        }

        return char::from_u32(unicode_val).unwrap();
    }

    pub fn print_board(self) {
        
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


}
