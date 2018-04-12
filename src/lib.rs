pub trait Environment {
    type Action;
    type State;

    fn new() -> Self::State;

    fn actions(state: &Self::State) -> Vec<Self::Action>;
    fn take_action(state: &Self::State, action: &Self::Action) -> (Self::State, f64);

    fn render(_state: &Self::State) {}
}

pub trait Agent<State, Action> {

    fn get_v(&self, state: &State) -> f64;
    fn update_v(&mut self, state: &State, value: f64);

    fn get_policy(&self, state: &State, actions: &[Action]) -> Action;
    // TODO updating the policy
}
