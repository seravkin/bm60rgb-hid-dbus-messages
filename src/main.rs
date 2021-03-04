use std::error::Error;
use std::sync::Arc;
use crate::application_state::ApplicationState;
use crate::dbus_watcher::watch_dbus;
use crate::device_render::spawn_render_thread;
use std::io::stdin;

mod keyboard_device;
mod application_state;
mod animation;
mod device_render;
mod dbus_watcher;

fn main() -> Result<(), Box<dyn Error>> {
    let application_state = Arc::new(ApplicationState::default());

    let render_handle = spawn_render_thread(application_state.clone())?;
    let dbus_handle = watch_dbus(application_state.clone())?;

    let mut any_key_buf = String::new();
    stdin().read_line(&mut any_key_buf)?;

    application_state.fire_exit_signal();

    // except because it can't be converted to boxed error
    dbus_handle.join().expect("Can't join dbus thread on exit");
    render_handle.join().expect("Can't join render thread on exit");

    Ok(())
}



