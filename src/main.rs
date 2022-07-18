mod game;

use game::Game;

fn main() {

    let g = Game::default();

    g.print_board();
}
