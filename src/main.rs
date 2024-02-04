use std::io::{self, Write};

use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    println!("\nPress q to quit, p to pause and resume.");
    tokio::spawn(countdown());
    let keypress_handle = tokio::spawn(keypress_listen());

    tokio::select! {
    _ = keypress_handle => {
        println!("\nKey press. Cancelling timer.");
    }
    }
}

/// Checks for keypresses every 100 ms and acts accordingly.
async fn keypress_listen() {
    let _raw = terminal::enable_raw_mode().unwrap();

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => {
                        println!("\nq key pressed. Exiting.");
                        break;
                    }
                    KeyCode::Char('p') => {}
                    _ => (),
                }
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
}

/// Prints the elapsed tiime every second.
async fn countdown() {
    let start_time = Local::now();
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
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
