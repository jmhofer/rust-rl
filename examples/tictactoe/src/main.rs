extern crate rl;

mod tictactoe;

use rl::*;
use tictactoe::*;

struct TicTacToeEnv {}

impl Environment for TicTacToeEnv {
    type Action = usize;
    type State = Game;

    fn new() -> Game {
        Game::new()
    }

    fn actions(state: &Game) -> Vec<usize> {
        state.actions()
    }

    fn take_action(state: &Game, action: &usize) -> (Game, f64) {
        (state.take_action(*action), 0.0) // TODO compute and return a proper reward
    }

    fn render(state: &Game) {
        println!("{}", state.render());
    }
}

fn main() {
    println!("Hello, world!");

    let game = TicTacToeEnv::new();

    TicTacToeEnv::render(&game);
}
