mod game;
mod ui;
mod player;

use game::Game;
use ui::ChessUI;

fn main() {

    let g = Game::default();

    let mut ui = ChessUI::from(g);

    ui.print_board();
    ui.print_legal_moves();
    ui.input_move();
    ui.print_board();
}
