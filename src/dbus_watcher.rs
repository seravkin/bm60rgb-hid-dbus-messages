use dbus::message::MatchRule;
use dbus::{MessageType, Message};
use std::time::Duration;
use crate::application_state::ApplicationState;
use std::sync::Arc;
use std::collections::HashMap;
use dbus::arg::{Variant, RefArg};
use dbus::blocking::Connection;
use dbus::channel::MatchingReceiver;
use std::thread::JoinHandle;

pub fn watch_dbus(state: Arc<ApplicationState>) -> Result<JoinHandle<()>, dbus::Error> {
    let conn = Connection::new_session()?;
    let mut rule = MatchRule::new();
    rule.msg_type = Some(MessageType::MethodReturn);

    let proxy = conn.with_proxy("org.freedesktop.DBus", "/org/freedesktop/DBus", Duration::from_millis(5000));
    let result: Result<(), dbus::Error> = proxy.method_call("org.freedesktop.DBus.Monitoring", "BecomeMonitor", (vec!(rule.match_str()), 0u32));
    let cloned_state = state.clone();

    if result.is_ok() {
        conn.start_receive(rule, Box::new(move |msg, _| {
            handle_message(&msg, state.clone());
            true
        }));
    } else {
        rule.eavesdrop = true;
        conn.add_match(rule, move |_: (), _, msg| {
            handle_message(&msg, state.clone());
            true
        })?;
    }

    Ok(std::thread::spawn(move || {
        loop {
            if cloned_state.has_exit_signal_fired() {
                break
            }
            match conn.process(Duration::from_millis(500)) {
                Ok(_) => {}
                Err(err) => {
                    println!("Error in dbus watcher: {}", err);
                    cloned_state.fire_exit_signal();
                    break
                }
            }
        }
    }))
}

fn handle_message(msg: &Message, state: Arc<ApplicationState>) {
    // Hack to get information about unread messages using only dbus
    if let Some(main_object) = msg.get1::<HashMap<&str, Variant<Box<dyn RefArg>>>>() {
        if let Some(icon_name) = main_object.get("IconName").and_then(|x| x.0.as_str()) {
            if icon_name == "telegram-attention-panel" {
                state.set_unread_telegram_message(true);
            } else if icon_name == "telegram-mute-panel" {
                state.set_unread_telegram_message(false);
            }
        }
    }
}

