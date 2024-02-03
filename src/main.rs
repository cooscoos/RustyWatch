use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use chrono::Local;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    // Create a shared cancellation token
    let cancel_token = Arc::new(AtomicBool::new(false));

    // Clone the token for the keypress task
    let keypress_token = Arc::clone(&cancel_token);

    // Spawn a task to listen for a keypress.
    let keypress_handle = tokio::spawn(async move {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        // Cancel the token when a key is pressed.
        keypress_token.store(true, Ordering::SeqCst);
    });

    // Spawn a task to print the elapsed time every second.
    tokio::spawn(async move {
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
    });

    tokio::select! {
    _ = keypress_handle => {
        println!("Key press. Cancelling timer.");
    }
    }
}
