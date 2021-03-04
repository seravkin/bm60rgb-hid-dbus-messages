use std::sync::atomic::{AtomicBool, Ordering};

pub struct ApplicationState {
    has_unread_telegram_message: AtomicBool,
    exit_signal: AtomicBool
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            has_unread_telegram_message: AtomicBool::new(false),
            exit_signal: AtomicBool::new(false)
        }
    }
}

impl ApplicationState {
    pub fn has_unread_telegram_message(&self) -> bool {
        self.has_unread_telegram_message.load(Ordering::Relaxed)
    }

    pub fn set_unread_telegram_message(&self, value: bool) {
        self.has_unread_telegram_message.store(value, Ordering::Relaxed);
    }

    pub fn fire_exit_signal(&self) {
        self.exit_signal.store(true, Ordering::Relaxed);
    }

    pub fn has_exit_signal_fired(&self) -> bool {
        self.exit_signal.load(Ordering::Relaxed)
    }
}