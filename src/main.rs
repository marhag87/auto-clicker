use device_query::{DeviceEvents, DeviceEventsHandler, DeviceQuery, DeviceState, Keycode};
use enigo::{Button, Direction, Enigo, Mouse, Settings};
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let is_clicking = Arc::new(AtomicBool::new(false));
    let is_clicking_clone = Arc::clone(&is_clicking);

    println!("Auto-clicker ready!");
    println!("Press 'F8' to toggle on/off.");

    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        loop {
            if is_clicking_clone.load(Ordering::Relaxed) {
                let _ = enigo.button(Button::Left, Direction::Click);
                thread::sleep(Duration::from_millis(1));
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
    });

    let device_events = DeviceEventsHandler::new(Duration::from_millis(10))
        .ok_or("Failed to start device listener")?;
    let _guard = device_events.on_key_down(move |key| {
        if *key == Keycode::F8 {
            let current_state = is_clicking.load(Ordering::Relaxed);
            is_clicking.store(!current_state, Ordering::Relaxed);
            println!("Clicker Active: {}", !current_state);
        }
    });
    thread::park();
    Ok(())
}
