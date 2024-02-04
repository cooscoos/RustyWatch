use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::time::Duration;

/// Checks for keypresses every 100 ms and acts accordingly.
pub async fn keypress_listen(pause: Arc<AtomicBool>, quit: Arc<AtomicBool>) {
    // Enable raw mode allows keypresses to be read without needing to press enter.
    terminal::enable_raw_mode().unwrap();

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                match event.code {
                    // 'q' press sets quit flag to true and exits
                    KeyCode::Char('q') => {
                        quit.store(true, Ordering::Relaxed);
                        terminal::disable_raw_mode().unwrap();
                        break;
                    }
                    // 'p' press toggles the pause flag
                    KeyCode::Char('p') => {
                        let current = pause.load(Ordering::Relaxed);
                        pause.store(!current, Ordering::Relaxed);
                    }
                    // Ignore all other keypresses
                    _ => {}
                }
            }
        }
    }
}

/// Prints elapsed time every second and handles pause toggles.
/// Returns the elapsed time when the function ends.
pub async fn elapsed_time(pause: Arc<AtomicBool>, quit: Arc<AtomicBool>) -> String {
    let mut start_time = Local::now();
    let mut pause_start_time: Option<chrono::DateTime<Local>> = None;
    let mut elapsed_string = String::new();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Stop and return elapsed time if 'q' button pressed
        if quit.load(Ordering::Relaxed) {
            break;
        }

        // Pause or resume timer if 'p' button pressed
        if pause.load(Ordering::Relaxed) {
            // Records pause start time
            if pause_start_time.is_none() {
                pause_start_time = Some(Local::now());
            }
        } else {
            // Calculates total pause duration
            if let Some(pause_start) = pause_start_time {
                start_time = start_time + (Local::now() - pause_start);
                pause_start_time = None;
            }
            let elapsed = Local::now() - start_time;
            elapsed_string = format!(
                "{:02}:{:02}:{:02}",
                elapsed.num_hours(),
                elapsed.num_minutes() % 60,
                elapsed.num_seconds() % 60
            );
            print!("\rElapsed time: {}", elapsed_string);
            io::stdout().flush().unwrap();
        }
    }

    elapsed_string
}
