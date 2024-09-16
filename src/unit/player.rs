#![allow(non_snake_case)]

pub mod Player {
    use std::fmt::{self, Display};
    use crate::{traits::Position, point::point::Point2d};
    
    pub struct Player {
        position: Point2d<f64>,
        speed: f64,
        health: u8,
        noclip: bool,
        speedlimit: bool,
        invincibility: bool,
    }
    impl Player {
        pub fn builder() -> PlayerBuilder {
            PlayerBuilder::default()
        }
        pub fn is_alive(&self) -> bool {
            self.health > 0
        }
        pub fn take_damage(&mut self, damage: u8) {
            if self.invincibility {
                return;
            }
            if self.health >= damage {
                self.health -= damage;
            } 
            else {
                self.health = 0;
            }
        }
        pub fn health(&self) -> u8 {
            self.health
        }
        pub fn speed(&self) -> f64 {
            self.speed
        }
        pub fn accelerate(&mut self) {
            if self.speed <= 0.9 || !self.speedlimit {
                self.speed += 0.1;
            }
            else {
                self.speed = 1.0;
            }
        }
        pub fn decelerate(&mut self) {
            if self.speed >= 0.1 || !self.speedlimit {
                self.speed -= 0.1;
            }
            else {
                self.speed = 0.0;
            }
        }
        pub fn move_forward(&mut self) {
            self.position.move_forward(self.speed);
        }
        pub fn forward_position(&self) -> Point2d<u16> {
            self.position.simulate_move_forward(self.speed).to_u16()
        }
        pub fn turn_left(&mut self) {
            self.position.update_direction(-1);
        }
        pub fn turn_right(&mut self) {
            self.position.update_direction(1);
        }
        pub fn toggle_noclip(&mut self) {
            self.noclip ^= true; 
        }
        pub fn toggle_speed(&mut self) {
            self.speedlimit ^= true;
        }
        pub fn noclip(&self) -> bool {
            self.noclip
        }
        pub fn unlimited_health(&mut self) {
            self.health = u8::MAX;
        }
        pub fn toggle_invincibility(&mut self) {
            self.invincibility ^= true;
        }
    }
    impl Default for Player {
        fn default() -> Self {
            Self {
                position: Point2d::new(30.0, 15.0),
                speed: 0.0,
                health: 10,
                noclip: false,
                speedlimit: true,
                invincibility: false,
            }
        }
    }
    impl Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let expression = "😭😱😨😫😩😧😖😞😑😐".chars().nth(self.health as usize - 1).unwrap_or_default();
            write!(f, "🔫{}", expression)
        }
    }
    impl Position<f64> for Player {
        fn position(&self) -> Point2d<f64> {
            self.position
        }
        fn set_position(&mut self, position: Point2d<f64>) {
            self.position = position;
        }
    }

    pub struct PlayerBuilder {
        position: Point2d<f64>,
        speed: f64,
        health: u8,
    }
    impl PlayerBuilder {
        pub fn new() -> PlayerBuilder {
            PlayerBuilder::default()
        }
        pub fn speed(mut self, speed: f64) -> PlayerBuilder {
            self.speed = speed;
            self
        }
        pub fn health(mut self, health: u8) -> PlayerBuilder {
            self.health = health;
            self
        }
        pub fn direction(mut self, x: f64, y: f64) -> PlayerBuilder {
            self.position.direction.0 = x;
            self.position.direction.1 = y;
            self
        }
        pub fn build(self) -> Player {
            Player {
                position: self.position,
                speed: self.speed,
                health: self.health, 
                noclip: false,
                speedlimit: true,
                invincibility: false,
            }
        }
    }
    impl Default for PlayerBuilder {
        fn default() -> Self {
            Self {
                position: Point2d::new(30.0, 15.0),
                speed: 0.0,
                health: 10,
            }
        }
    }
}
