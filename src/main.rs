use crate::game::Game;

mod game;

fn main() {
    let mut game = Game::start_game(25, 25);

    loop {
        let state = game.get_state();
        let action = ai.decide(state);
        ai.remember_action(action);

        let reward = game.step(action);
        let next_state = game.get_state();

        ai.learn(next_state, reward);

        if game.is_over() {
            break;
        }
    }

    for episode in 0..10000 {
        let mut game = Game::new();
        while !game.is_over() {
            let state = game.encode_state();
            let action = ai.decide(&state);
            let reward = game.step(action);
            ai.learn(state, action, reward);
        }
    }
}
