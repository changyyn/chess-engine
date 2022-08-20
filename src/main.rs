mod game;
mod ui;
mod player;
mod util;

use game::Game;
use ui::ChessUI;

fn main() {

    let g = Game::default();

    let mut ui = ChessUI::from(g);

    ui.print_board();
    ui.print_legal_moves();
    ui.take_move();
    ui.print_board();
}
