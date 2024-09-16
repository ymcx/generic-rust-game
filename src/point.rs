pub mod point {
    use std::ops::Sub;
    use std::fmt::{Display, Formatter, Result};
    use num::Float;

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct Point2d<T> {
        pub x: T,
        pub y: T,
        pub direction: (f64, f64, i8),
    }
    impl<T> Point2d<T> {
        pub fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
                direction: (1.00, 0.00, 0),
            }
        }
    }
    impl<T: Display + Float> Display for Point2d<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "P:{}x{} D:{}", self.x.round(), self.y.round(), self.direction.2)
        }
    }
    impl<T: Default> Default for Point2d<T> {
        fn default() -> Self {
            Self::new(
                T::default(),
                T::default(),
            )
        }
    }
    impl<T: Sub<Output = T>> Sub for Point2d<T> {
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                direction: (
                    self.direction.0 - other.direction.0,
                    self.direction.1 - other.direction.1,
                    self.direction.2 - other.direction.2,
                ),
            }
        }
    }
    impl Point2d<f64> {
        pub fn move_forward(&mut self, speed: f64) {
            self.x += self.direction.0 * speed;
            self.y += self.direction.1 * speed;
        }
        pub fn simulate_move_forward(&self, speed: f64) -> Self {
            Self::new(
                self.x + self.direction.0 * speed,
                self.y + self.direction.1 * speed,
            )
        }
        pub fn update_direction(&mut self, turn: i8) {
            self.direction.2 = (8 + self.direction.2 + turn) % 8;
            (self.direction.0, self.direction.1) = match self.direction.2 {
                0 => ( 1.0000,  0.0000),
                1 => ( 0.7061,  0.7061),
                2 => ( 0.0000,  1.0000),
                3 => (-0.7061,  0.7061),
                4 => (-1.0000,  0.0000),
                5 => (-0.7061, -0.7061),
                6 => ( 0.0000, -1.0000),
                7 => ( 0.7061, -0.7061),
                _ => ( 0.0000,  0.0000),
            }
        }
        pub fn to_u16(&self) -> Point2d<u16> {
            Point2d::new(
                self.x as u16,
                self.y as u16,
            )
        }
        pub fn round(&self) -> Self {
            Self::new(
                self.x.round(),
                self.y.round(),
            )
        }
    }
}
