use std::io::{self, Write};

use chrono::Local;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    tokio::spawn(countdown());
    let keypress_handle = tokio::spawn(keypress_listenener());

    tokio::select! {
    _ = keypress_handle => {
        println!("Key press. Cancelling timer.");
    }
    }
}

/// Waits and listens for a keypress.
async fn keypress_listenener() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

/// Prints the elapsed tiime every second.
async fn countdown() {
    let start_time = Local::now();
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let elapsed = Local::now() - start_time;
        print!(
            "\rElapsed time:{}h{}m{}s",
            elapsed.num_hours(),
            elapsed.num_minutes(),
            elapsed.num_seconds()
        );
        io::stdout().flush().unwrap();
    }
}
