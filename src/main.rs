mod game;
mod ui;

use game::Game;
use ui::ChessUI;

fn main() {

    let g = Game::default();

    let ui = ChessUI::from(g);

    ui.print_board();
}
