use std::sync::Arc;
use crate::application_state::ApplicationState;
use std::thread::JoinHandle;
use std::error::Error;
use crate::keyboard_device::{Keyboard, KeyboardState};
use crate::animation::{Tick, render};
use std::time::{Instant, Duration};

const ANIMATION_TICK_IN_MS: u64 = 250;

pub fn spawn_render_thread(app_state: Arc<ApplicationState>) -> Result<JoinHandle<()>, Box<dyn Error>> {
    let keyboard = Keyboard::new()?;

    keyboard.clear_all_keys()?;

    Ok(std::thread::spawn(move || {
        let mut keyboard_state = KeyboardState::default();
        let mut tick_count = Tick::default();

        loop {
            if app_state.has_exit_signal_fired() {
                break
            }

            let now = Instant::now();

            keyboard_state.clear_updates();
            keyboard_state = render(&app_state, tick_count, keyboard_state);

            match keyboard.light_keys(keyboard_state.get_updates()) {
                Ok(_) => {}
                Err(err) => {
                    println!("Error in render thread: {}", err);
                    app_state.fire_exit_signal();
                    break;
                }
            }

            let elapsed = now.elapsed().as_millis() as u64;
            std::thread::sleep(Duration::from_millis(ANIMATION_TICK_IN_MS - elapsed));
            tick_count.increment();
        }
    }))
}