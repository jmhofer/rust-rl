#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    X,
    O
}

impl Color {
    pub fn switch(self) -> Color {
        match self {
            Color::X => Color::O,
            Color::O => Color::X
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GameResult {
    pub has_ended: bool,
    pub winner: Option<Color>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Game {
    pub board: [Option<Color>; 9],
    pub to_move: Color
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [None; 9],
            to_move: Color::X
        }
    }

    pub fn take_action(&self, action: usize) -> Game {
        let mut next = self.clone();
        next.board[action] = Some(next.to_move);
        next.to_move = next.to_move.switch();
        next
    }

    pub fn actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for i in 0..9usize {
            if self.board[i].is_none() {
                actions.push(i)
            }
        }
        actions
    }

    pub fn render(&self) -> String {
        let mut board_str = String::new();
        for i in 0..9usize {
            board_str += match self.board[i] {
                None => " . ",
                Some(Color::X) => " X ",
                Some(Color::O) => " O "
            };
            if i % 3 == 2 {
                board_str += "\n";
            }
        }
        board_str
    }

    pub fn take_actions(&self, actions: &[usize]) -> Game {
        if actions.is_empty() {
            self.clone()
        } else {
            self.take_action(actions[0]).take_actions(&actions[1..])
        }
    }

    pub fn result(&self) -> GameResult {
        if self.actions().is_empty() {
            GameResult { has_ended: true, winner: None }
        } else {
            for i in 0..3usize {
                if self.board[i].is_some() && self.board[i] == self.board[3 + i] && self.board[i] == self.board[6 + i] {
                    return GameResult { has_ended: true, winner: self.board[i] };
                }
                if self.board[3 * i].is_some() && self.board[3 * i] == self.board[3 * i + 1] && self.board[3 * i] == self.board[3 * i + 2] {
                    return GameResult { has_ended: true, winner: self.board[i] };
                }
            }
            if self.board[4].is_some() && (self.board[0] == self.board[4] && self.board[8] == self.board[4] || self.board[2] == self.board[4] && self.board[6] == self.board[4]) {
                return GameResult { has_ended: true, winner: self.board[4] };
            }
            GameResult { has_ended: false, winner: None }
        }
    }

    pub fn hash(&self) -> u16 {
        let mut hash: u16 = 0;
        let mut power: u16 = 1;
        for &b in self.board.iter() {
            hash += power * (match b {
                None => 0,
                Some(Color::X) => 1,
                Some(Color::O) => 2
            });
            power *= 3;
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use tictactoe::*;

    #[test]
    fn test_actions() {
        let initial = Game::new();
        assert_eq!(initial.actions(), vec!(0, 1, 2, 3, 4, 5, 6, 7, 8));

        let game = initial.take_actions(&[4, 0, 1, 7]);
        assert_eq!(game.actions(), vec!(2, 3, 5, 6, 8));
    }

    #[test]
    fn test_hash() {
        let initial = Game::new();
        assert_eq!(initial.hash(), 0);

        let game = initial.take_actions(&[4, 0, 1, 7]);
        assert_eq!(game.hash(), 1 * 3u16.pow(4) + 2 * 3u16.pow(0) + 1 * 3u16.pow(1) + 2 * 3u16.pow(7));
    }

    #[test]
    fn test_result() {
        let initial = Game::new();
        assert_eq!(initial.result(), GameResult { has_ended: false, winner: None });

        let x_win = initial.take_actions(&[4, 0, 1, 2, 7]);
        assert_eq!(x_win.result(), GameResult { has_ended: true, winner: Some(Color::X) });

        let o_win = initial.take_actions(&[4, 0, 1, 3, 2, 6]);
        assert_eq!(o_win.result(), GameResult { has_ended: true, winner: Some(Color::O) });

        let draw = initial.take_actions(&[4, 0, 2, 6, 3, 5, 1, 7, 8]);
        assert_eq!(draw.result(), GameResult { has_ended: true, winner: None });
    }

    #[test]
    fn test_render() {
        let initial = Game::new();
        assert_eq!(initial.render(), " .  .  . \n .  .  . \n .  .  . \n");

        let game = initial.take_actions(&[4, 0, 1, 7]);
        assert_eq!(game.render(), " O  X  . \n .  X  . \n .  O  . \n");
    }
}
