use std::time::Duration;

use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

use crate::unit::Player;

pub fn poll_key_event(duration: Duration) -> Option<KeyEvent> {
    if poll(duration).ok()? {
        let event = crossterm::event::read().ok()?;
        if let Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                return Some(key_event);
            }
        }
    }
    None
}

pub fn handle_key_event(key: KeyEvent, player: &mut Player, quit: &mut bool) {
    match key.code {
        KeyCode::Left => player.turn_left(),
        KeyCode::Right => player.turn_right(),
        KeyCode::Up => player.accelerate(),
        KeyCode::Down => player.decelerate(),
        KeyCode::Char('n') => player.toggle_noclip(),
        KeyCode::Char('s') => player.toggle_speed(),
        KeyCode::Char('u') => player.toggle_invincibility(),
        KeyCode::Char('q') | KeyCode::Esc => *quit = true,
        KeyCode::Char('c') => {
            if key.modifiers == KeyModifiers::CONTROL {
                *quit = true;
            }
        }
        _ => {}
    }
}

