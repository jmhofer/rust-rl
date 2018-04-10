extern crate rand;
extern crate rl;

mod tictactoe;

use rl::*;
use tictactoe::*;
use rand::{Rng, thread_rng};
use std::collections::HashMap;

struct TicTacToeEnv {}

impl Environment for TicTacToeEnv {
    type Action = usize;
    type State = Game;

    fn new() -> Game {
        Game::new()
    }

    fn actions(game: &Game) -> Vec<usize> {
        game.actions()
    }

    fn take_action(game: &Game, action: &usize) -> (Game, f64) {
        (game.take_action(*action), 0.0) // TODO compute and return a proper reward
    }

    fn render(game: &Game) {
        println!("{}", game.render());
    }
}

const EPSILON: f64 = 0.1;

struct TicTacToeAgent {
    v: HashMap<u16, f64>
}

impl Agent<Game, usize> for TicTacToeAgent {
    fn get_q(&self, game: Game, action: &usize) -> f64 {
        *self.v.get(&game.take_action(*action).hash()).unwrap_or(&0.0)
    }

    fn update_q(&mut self, game: Game, action: &usize, value: f64) {
        self.v.insert(game.take_action(*action).hash(), value);
    }

    // epsilon-greedy policy
    fn get_policy(&self, game: Game, actions: &[usize]) -> usize {
        let mut rng = thread_rng();
        if rng.gen::<f64>() < EPSILON {
            *rng.choose(actions).unwrap()
        } else {
            let mut chosen = actions[0];
            let mut max_q = self.get_q(game, &chosen);
            for action in actions[1..].iter() {
                let q = self.get_q(game, action);
                if q > max_q {
                    max_q = q;
                    chosen = *action;
                }
            }
            chosen
        }
    }
}

fn main() {
    println!("Hello, world!");

    let game = TicTacToeEnv::new();

    TicTacToeEnv::render(&game);
}
