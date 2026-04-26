mod opts;

use crate::opts::Opts;
use clap::Parser;
use device_query::DeviceEvents;
use device_query::DeviceEventsHandler;
use device_query::Keycode;
use enigo::Button;
use enigo::Direction;
use enigo::Enigo;
use enigo::Mouse;
use enigo::Settings;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let is_clicking = Arc::new(AtomicBool::new(false));
    let is_clicking_clone = Arc::clone(&is_clicking);
    println!("Auto-clicker ready!");
    let device_events = DeviceEventsHandler::new(Duration::from_millis(10))
        .ok_or("Failed to start device listener")?;
    if let Some(mouse_key) = opts.mouse {
        println!("Press mouse '{mouse_key}' to toggle on/off.");
        let _guard = device_events.on_mouse_up(move |key| {
            if *key == mouse_key {
                click(is_clicking_clone.clone());
            }
        });
        thread::park();
    } else {
        let keyboard_key = opts.keyboard.unwrap_or(Keycode::F8);
        println!("Press '{keyboard_key}' to toggle on/off.");
        let _guard = device_events.on_key_up(move |key| {
            if *key == keyboard_key {
                click(is_clicking_clone.clone());
            }
        });
        thread::park();
    }
    Ok(())
}

fn click(is_clicking: Arc<AtomicBool>) {
    let was_clicking = is_clicking.fetch_xor(true, Ordering::SeqCst);
    if !was_clicking {
        println!("Clicker: ON");
        let thread_flag = Arc::clone(&is_clicking);
        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).expect("Enigo init failed");
            while thread_flag.load(Ordering::SeqCst) {
                let _ = enigo.button(Button::Left, Direction::Click);
                thread::sleep(Duration::from_millis(18));
            }
            println!("Clicker: OFF");
        });
    }
}
