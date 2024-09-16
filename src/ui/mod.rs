pub mod draw;

use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{self, MoveTo},
    execute, queue,
    style::{Color, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct UI {
    stdout: Stdout,
}

impl UI {
    pub fn new() -> Self {
        let stdout = stdout();
        Self { stdout }
    }

    pub fn prepare(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            self.stdout,
            SetBackgroundColor(Color::Black),
            Clear(ClearType::All),
            cursor::Hide,
        )
        .unwrap();
    }

    pub fn clear(&mut self) {
        queue!(self.stdout, Clear(ClearType::All)).unwrap();
    }

    pub fn restore(&mut self) {
        execute!(
            self.stdout,
            ResetColor,
            Clear(ClearType::All),
            MoveTo(0, 0),
            cursor::Show
        )
        .unwrap();
        disable_raw_mode().unwrap();
    }
}
