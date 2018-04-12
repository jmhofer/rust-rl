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

    fn self_play(&mut self, should_render: bool, epsilon: f64) -> Option<Color> {
        let mut game = TicTacToeEnv::new();
        self.epsilon = epsilon;
        while !game.result.has_ended {
            let my_color = game.to_move;
            let next_actions = game.actions();
            let chosen_action = self.get_policy(&game, &next_actions);
            let (next_turn, reward) = TicTacToeEnv::take_action(&game, &chosen_action);

            // s_t   = game
            // r_t   = reward
            // s_t+1 = next_turn

            // println!("reward: {}", agent_reward);

            let new_v = if next_turn.result.has_ended {
                reward
            } else {
                reward + (1.0 - self.gamma) * self.get_v(&game) + self.gamma * self.get_v(&next_turn)
            };

            // println!("{} -> {}", self.get_v(&game), new_v);
            self.update_v(&game, new_v);

            game = next_turn;

            if should_render {
                TicTacToeEnv::render(&game);
            }
        }
        game.result.winner
    }
}

impl Agent<Game, usize> for TicTacToeAgent {
    fn get_v(&self, game: &Game) -> f64 {
        *self.v.get(&game.hash()).unwrap_or(&0.0)
    }

    fn update_v(&mut self, game: &Game, value: f64) {
        self.v.insert(game.hash(), value);
    }

    // epsilon-greedy policy
    fn get_policy(&self, game: &Game, actions: &[usize]) -> usize {
        let mut rng = thread_rng();
        if rng.gen::<f64>() < self.epsilon {
            *rng.choose(actions).unwrap()
        } else {
            let mut chosen = actions[0];
            let mut best_v = self.get_v(&game.take_action(chosen));
            for &action in actions[1..].iter() {
                let v = self.get_v(&game.take_action(action));
                if game.to_move == Color::X && v > best_v || game.to_move == Color::O && v < best_v {
                    best_v = v;
                    chosen = action;
                }
            }
            chosen
        }
    }
}

fn main() {
    let mut agent = TicTacToeAgent::new(0.9, 0.2);
    for _i in 0..100000 {
        agent.self_play(false, 0.2);
    }
    let mut draws = 0;
    for _i in 0..1000 {
        if agent.self_play(true, 0.0).is_none() {
            draws += 1;
        }
    }
    println!("draws: {}", draws);
}
