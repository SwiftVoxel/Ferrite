mod logger;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    log_info!("main", "Starting Ferrite...");
    log_info!("main", "Loading Ferrite version 0.1.0");

    // Simulated startup sequence - this is where world/render init will
    // eventually hook in. For now we're console-only.
    thread::sleep(Duration::from_millis(150));
    log_info!("main", "Preparing level \"world\"");

    thread::sleep(Duration::from_millis(150));
    log_info!("main", "Done! Type \"help\" for a list of commands.");

    run_console();

    log_info!("main", "Stopping!");
}

fn run_console() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().ok();

        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap_or(0) == 0 {
            // EOF (e.g. piped input ended)
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input {
            "stop" | "exit" | "quit" => {
                break;
            }
            "help" => {
                log_info!("main", "Known commands: help, stop");
            }
            other => {
                log_warn!("main", "Unknown command: \"{}\"", other);
            }
        }
    }
}
