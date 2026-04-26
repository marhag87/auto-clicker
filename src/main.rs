use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Button, Direction, Enigo, Mouse, Settings};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
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

    let device_state = DeviceState::new();
    let mut was_f8_pressed = false;

    loop {
        let keys = device_state.get_keys();
        let is_f8_pressed = keys.contains(&Keycode::F8);
        if is_f8_pressed && !was_f8_pressed {
            let current_state = is_clicking.load(Ordering::Relaxed);
            is_clicking.store(!current_state, Ordering::Relaxed);
            println!("Clicker Active: {}", !current_state);
        }
        was_f8_pressed = is_f8_pressed;
        thread::sleep(Duration::from_millis(10));
    }
}
