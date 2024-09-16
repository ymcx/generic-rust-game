#![allow(non_snake_case)]

pub mod Wall {
    use crate::{traits::Position, point::point::Point2d};
    
    pub struct Wall {
        position: Point2d<u16>,
    }
    impl Wall {
        pub fn new(x: u16, y: u16) -> Wall {
            Self {
                position: Point2d::new(x, y),
            }
        }
    }
    impl Default for Wall {
        fn default() -> Self {
            Self::new(
                0,
                0,
            )
        }
    }
    impl Position<u16> for Wall {
        fn position(&self) -> Point2d<u16> {
            self.position
        }
        fn set_position(&mut self, position: Point2d<u16>) {
            self.position = position;
        }
    }
    impl Position<f64> for Wall {
        fn position(&self) -> Point2d<f64> {
            Point2d::new(
                self.position.x as f64,
                self.position.y as f64,
            )
        }
        fn set_position(&mut self, position: Point2d<f64>) {
            self.position.x = position.x as u16;
            self.position.y = position.y as u16;
        }
    }
}
