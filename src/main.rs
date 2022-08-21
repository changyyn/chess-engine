mod game;
mod ui;
mod player;
mod util;

use game::Game;
use ui::ChessUI;

fn main() {

    let g = Game::default();
    let mut ui = ChessUI::from(g);

    ui.run();
}
