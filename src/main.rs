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

#[tokio::main]
async fn main() {
    println!("\nPress 'q' to quit, or 'p' to pause and resume the timer.");

    let is_paused = Arc::new(AtomicBool::new(false));
    tokio::spawn(elapsed_time(is_paused.clone()));
    let keypress_handle = tokio::spawn(keypress_listen(is_paused));

    tokio::select! {
    _ = keypress_handle => {
        println!("\nExited.");
    }
    }
}

/// Checks for keypresses every 100 ms and acts accordingly.
async fn keypress_listen(pause: Arc<AtomicBool>) {
    // raw mode allows keypresses to be read without needing to press enter.
    terminal::enable_raw_mode().unwrap();

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') => {
                        let current = pause.load(Ordering::Relaxed);
                        pause.store(!current, Ordering::Relaxed);
                    }
                    _ => (),
                }
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
}

/// Prints elapsed time every second and handles pauses.
async fn elapsed_time(pause: Arc<AtomicBool>) {
    let mut start_time = Local::now();
    let mut pause_start_time: Option<chrono::DateTime<Local>> = None;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let is_paused = pause.load(Ordering::Relaxed);
        if is_paused {
            if pause_start_time.is_none() {
                pause_start_time = Some(Local::now());
            }
            continue;
        }
        if let Some(pause_start) = pause_start_time {
            start_time = start_time + (Local::now() - pause_start);
            pause_start_time = None;
        }
        let elapsed = Local::now() - start_time;
        print!(
            "\rElapsed time: {}h{}m{}s",
            elapsed.num_hours(),
            elapsed.num_minutes(),
            elapsed.num_seconds()
        );
        io::stdout().flush().unwrap();
    }
}
