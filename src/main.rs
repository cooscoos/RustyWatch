use std::{
    io::Write,
    sync::{atomic::AtomicBool, Arc},
};

use chrono::Local;
use rusty_watch::*;

/// Help message printed at the start of the program
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
            let elapsed_string = elapsed_time_handle.await.unwrap();

           // Write the elapsed time and the current timestamp to the log file
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("log.txt")
                .unwrap();

            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S,%a").to_string();
            writeln!(file, "{},{}", timestamp, elapsed_string).unwrap();

            println!("\r\nExited and saved to log");
        }
    }
}
