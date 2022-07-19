use chess::{Board, Rank, File};

pub struct Game {
   board: Board,
}

impl Game {

    pub const RANKS : [Rank; 8] = [Rank::First, Rank::Second, Rank::Third, Rank::Fourth, Rank::Fifth, Rank::Sixth, Rank::Seventh, Rank::Eighth];
    pub const FILES : [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];

    pub fn default() -> Game {
        return Game { board: (Board::default())}
    }

    pub fn get_board(self) -> Board {
        self.board.clone()
    }


}

