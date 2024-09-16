use crate::{
    hud::Hud::Hud,
    impl_display,
    traits::Position,
    unit::{Collectible, Enemy, Player, Wall},
};
use std::{fmt::Display, io::Write};

use crossterm::style::Stylize;
use num::{traits::NumAssign, NumCast};

pub trait Draw<T: NumAssign + Copy + NumCast + Default>: Position<T> + Display {
    fn draw(&self, stdout: &mut impl Write) {
        let position = self.position();
        crossterm::queue!(
            stdout,
            crossterm::cursor::MoveTo(
                position
                    .x
                    .to_f64()
                    .expect("could not convert position x to f64")
                    .round() as u16,
                position
                    .y
                    .to_f64()
                    .expect("could not convert position y to f64")
                    .round() as u16,
            ),
            crossterm::style::Print(self)
        )
        .unwrap();
    }
}

impl Draw<f64> for Player {}

impl Draw<u16> for Collectible {}
impl_display!(for Collectible: '❤'.red());

impl Draw<u16> for Wall {}
impl_display!(for Wall: '▓'.magenta());

impl Draw<f64> for Enemy {}
impl_display!(for Enemy: '⁂'.dark_green());

impl Draw<u16> for Hud<'_> {}
impl_display!(for Hud<'_>: method text);

