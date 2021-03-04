use crate::keyboard_device::{KeyboardState, T_KEY, BLUE, BLACK};
use crate::application_state::ApplicationState;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tick(usize);

impl Default for Tick {
    fn default() -> Self {
        Tick(0)
    }
}

impl Tick {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

fn blink(tick: Tick, mut keyboard_state: KeyboardState) -> KeyboardState {
    const ON_PERIOD: usize = 4;
    const OFF_PERIOD: usize = 4;

    if tick.0 % (ON_PERIOD + OFF_PERIOD) < ON_PERIOD {
        keyboard_state.set(T_KEY, BLUE);
    } else {
        keyboard_state.set(T_KEY, BLACK);
    }

    keyboard_state
}

fn always_dim(mut keyboard_state: KeyboardState) -> KeyboardState {
    keyboard_state.set(T_KEY, BLACK);
    keyboard_state
}

pub fn render(app_state: &ApplicationState, tick: Tick, mut keyboard_state: KeyboardState) -> KeyboardState {
    let needs_blinking = app_state.has_unread_telegram_message();

    if needs_blinking {
        keyboard_state = blink(tick, keyboard_state);
    } else {
        keyboard_state = always_dim(keyboard_state);
    }

    keyboard_state
}