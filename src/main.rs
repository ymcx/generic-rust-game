use application::{game, unit::Enemy};

fn main() {
    let mut game = game::Game::builder()
        .n_random_walls(30)
        .height(40)
        .player_starting_health(10)
        .player_starting_speed(0.5)
        .enemies(
            (1..12)
                .map(|i| Enemy::with_speed(i as f64 * 0.05))
                .collect(),
        )
        .update_interval(std::time::Duration::from_millis(70))
        .build();
    game.run();
}
