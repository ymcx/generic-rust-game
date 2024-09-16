#![allow(non_snake_case, dead_code)]

pub mod Hud {
    use crate::{point::point::Point2d, traits::Position, unit::player::Player::Player};

    pub struct Hud<'a> {
        score: u32,
        player: &'a Player,
        y_position: u16,
    }
    impl<'a> Hud<'a> {
        pub fn new(score: u32, player: &'a Player, y_position: u16) -> Self {
            Self {
                score,
                player,
                y_position,
            }
        }
        pub fn text(&self) -> String {
            format!("//\\\\//\\\\//\\\\//\\\\//  SCORE: {:2}  //  HEALTH: {:2}  \\\\  SPEED: {:4.1}  \\\\//\\\\//\\\\//\\\\//\\\\", self.score, self.player.health(), self.player.speed())
        }
    }
    impl<'a> Position<u16> for Hud<'a> {
        fn position(&self) -> Point2d<u16> {
            Point2d::new(
                0,
                self.y_position
            )
        }
        fn set_position(&mut self, position: Point2d<u16>) {
            self.y_position = position.y;
        }
    }
}
