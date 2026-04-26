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
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use std::time::Instant;

macro_rules! reprint {
    ($($arg:tt)*) => {
        print!("\r\x1B[2K{}", format_args!($($arg)*));
        io::stdout().flush().unwrap();
    };
}

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
                click(is_clicking_clone.clone(), opts.clicks_per_second);
            }
        });
        thread::park();
    } else {
        let keyboard_key = opts.keyboard.unwrap_or(Keycode::F8);
        println!("Press '{keyboard_key}' to toggle on/off.");
        let _guard = device_events.on_key_up(move |key| {
            if *key == keyboard_key {
                click(is_clicking_clone.clone(), opts.clicks_per_second);
            }
        });
        thread::park();
    }
    Ok(())
}

fn click(is_clicking: Arc<AtomicBool>, target_cps: u32) {
    let was_clicking = is_clicking.fetch_xor(true, Ordering::SeqCst);
    if !was_clicking {
        reprint!("Clicker: ON");
        let thread_flag = Arc::clone(&is_clicking);
        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).expect("Enigo init failed");
            let interval = Duration::from_secs(1) / target_cps;
            while thread_flag.load(Ordering::SeqCst) {
                let start_time = Instant::now();
                let _ = enigo.button(Button::Left, Direction::Click);
                let elapsed = start_time.elapsed();
                if elapsed < interval {
                    spin_sleep::sleep(interval - elapsed);
                }
            }
            reprint!("\rClicker: OFF");
        });
    }
}
