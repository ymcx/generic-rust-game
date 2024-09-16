#![allow(non_snake_case)]

pub mod Enemy {
    use crate::{point::point::Point2d, traits::Position, unit::Player};
    
    pub struct Enemy {
        position: Point2d<f64>,
        speed: f64,
    }
    impl Enemy {
        pub fn with_speed(speed: f64) -> Self {
            Self {
                position: Point2d::default(),
                speed,
            }
        }
        pub fn move_towards_player(&mut self, player: &Player) {
            let change = player.position() - self.position();
            let length = (f64::powi(change.x, 2)+f64::powi(change.y, 2)).sqrt();
            self.position.x += self.speed * change.x / length;
            self.position.y += self.speed * change.y / length;
        }
    }
    impl Default for Enemy {
        fn default() -> Self {
            Self {
                position: Point2d::default(),
                speed: 0.0,
            }
        }
    }
    impl Position<f64> for Enemy {
        fn position(&self) -> Point2d<f64> {
            self.position
        }
        fn set_position(&mut self, position: Point2d<f64>) {
            self.position = position;
        }
    }
}
