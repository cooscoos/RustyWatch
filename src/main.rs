use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use tokio::time::Duration;

const HELP: &str = r#"Press 'q' to quit, or 'p' to pause and resume the timer."#;

#[tokio::main]
async fn main() {
    println!("{}", HELP);

    let pause = Arc::new(AtomicBool::new(false));
    let quit = Arc::new(AtomicBool::new(false));
    let elapsed_time_handle = tokio::spawn(elapsed_time(pause.clone(), quit.clone()));
    let keypress_handle = tokio::spawn(keypress_listen(pause, quit));

    tokio::select! {
        _ = keypress_handle => {
            let elapsed = elapsed_time_handle.await.unwrap();
            println!("\r\nExited. Total elapsed time: {:?}", elapsed);
        }
    }
}

/// Checks for keypresses every 100 ms and acts accordingly.
async fn keypress_listen(pause: Arc<AtomicBool>, quit: Arc<AtomicBool>) {
    // raw mode allows keypresses to be read without needing to press enter.
    terminal::enable_raw_mode().unwrap();

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                match event.code {
                    KeyCode::Char('q') => {
                        // Set the quit AtomicBool to true when 'q' is pressed
                        quit.store(true, Ordering::Relaxed);
                        terminal::disable_raw_mode().unwrap();
                        break;
                    }
                    KeyCode::Char('p') => {
                        let current = pause.load(Ordering::Relaxed);
                        pause.store(!current, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Prints elapsed time every second and handles pause toggles.
/// Returns the elapsed time when the function ends.
async fn elapsed_time(pause: Arc<AtomicBool>, quit: Arc<AtomicBool>) -> chrono::Duration {
    let mut start_time = Local::now();
    let mut pause_start_time: Option<chrono::DateTime<Local>> = None;
    let mut elapsed = chrono::Duration::zero();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Stop and return elapsed time if 'q' button pressed
        if quit.load(Ordering::Relaxed) {
            break;
        }

        // Pause or resume if 'p' button pressed
        if pause.load(Ordering::Relaxed) {
            if pause_start_time.is_none() {
                pause_start_time = Some(Local::now());
            }
        } else {
            if let Some(pause_start) = pause_start_time {
                start_time = start_time + (Local::now() - pause_start);
                pause_start_time = None;
            }
            elapsed = Local::now() - start_time;
            print!(
                "\rElapsed time: {:02}:{:02}:{:02}",
                elapsed.num_hours(),
                elapsed.num_minutes() % 60,
                elapsed.num_seconds() % 60
            );
            io::stdout().flush().unwrap();
        }
    }

    elapsed
}
