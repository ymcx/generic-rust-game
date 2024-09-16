#![allow(non_snake_case)]

pub mod Collectible {
    use crate::{traits::Position, point::point::Point2d};

    pub struct Collectible {
        position: Point2d<u16>,
    }
    impl Collectible {
        pub fn new(x: u16, y: u16) -> Collectible {
            Self {
                position: Point2d::new(x, y),
            }
        }
    }
    impl Default for Collectible {
        fn default() -> Self {
            Self::new(
                0,
                0,
            )
        }
    }
    impl Position<u16> for Collectible {
        fn position(&self) -> Point2d<u16> {
            self.position
        }
        fn set_position(&mut self, position: Point2d<u16>) {
            self.position = position;
        }
    }
}
