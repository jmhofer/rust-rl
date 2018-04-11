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

    // always rewards from the point of view of the 'X' player
    fn take_action(game: &Game, action: &usize) -> (Game, f64) {
        let next = game.take_action(*action);
        let reward = match next.result {
            GameResult { has_ended: false, winner: _ } => 0.0,
            GameResult { has_ended: true, winner: None } => 0.0,
            GameResult { has_ended: true, winner: Some(Color::X) } => 1.0,
            GameResult { has_ended: true, winner: Some(Color::O) } => -1.0
        };
        (next, reward)
    }

    fn render(game: &Game) {
        println!("{}", game.render());
    }
}

struct TicTacToeAgent {
    v: HashMap<u16, f64>,
    gamma: f64,
    epsilon: f64
}

impl TicTacToeAgent {
    fn new(gamma: f64, epsilon: f64) -> TicTacToeAgent {
        TicTacToeAgent {
            v: HashMap::new(),
            gamma,
            epsilon
        }
    }

    fn self_play(&mut self, should_render: bool) {
        let mut game = TicTacToeEnv::new();
        while !game.result.has_ended {
            let my_color = game.to_move;
            let next_actions = game.actions();
            let chosen_action = self.get_policy(&game, &next_actions);
            let (next_turn, reward) = TicTacToeEnv::take_action(&game, &chosen_action);
            game = next_turn;
            // TODO update Q(s, a)
            if should_render {
                TicTacToeEnv::render(&game);
            }
        }
    }
}

impl Agent<Game, usize> for TicTacToeAgent {
    fn get_q(&self, game: &Game, action: &usize) -> f64 {
        *self.v.get(&game.take_action(*action).hash()).unwrap_or(&0.0)
    }

    fn update_q(&mut self, game: &Game, action: &usize, value: f64) {
        self.v.insert(game.take_action(*action).hash(), value);
    }

    // epsilon-greedy policy
    fn get_policy(&self, game: &Game, actions: &[usize]) -> usize {
        let mut rng = thread_rng();
        if rng.gen::<f64>() < self.epsilon {
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

    let mut agent = TicTacToeAgent::new(0.9, 0.1);
    agent.self_play(true);
}
