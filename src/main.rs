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
    let is_clicking = Arc::new(AtomicBool::new(false));
    let is_clicking_clone = Arc::clone(&is_clicking);

    println!("Auto-clicker ready!");
    println!("Press 'F8' to toggle on/off.");

    let device_events = DeviceEventsHandler::new(Duration::from_millis(10))
        .ok_or("Failed to start device listener")?;
    let _guard = device_events.on_key_up(move |key| {
        if *key == Keycode::F8 {
            let was_clicking = is_clicking_clone.fetch_xor(true, Ordering::SeqCst);
            let now_clicking = !was_clicking;

            if now_clicking {
                println!("Clicker: ON");
                let thread_flag = Arc::clone(&is_clicking_clone);
                thread::spawn(move || {
                    let mut enigo = Enigo::new(&Settings::default()).expect("Enigo init failed");

                    // The thread lives ONLY as long as the flag is true
                    while thread_flag.load(Ordering::SeqCst) {
                        let _ = enigo.button(Button::Left, Direction::Click);
                        thread::sleep(Duration::from_millis(18));
                    }
                    println!("Clicker: OFF");
                });
            }
        }
    });
    thread::park();
    Ok(())
}
