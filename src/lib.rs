#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait Environment {
    type Action: Eq;
    type State: Eq;

    fn new() -> Self::State;

    fn actions(state: &Self::State) -> Vec<Self::Action>;
    fn take_action(state: &Self::State, action: &Self::Action) -> (Self::State, f64);

    fn render(state: &Self::State) {}
}
