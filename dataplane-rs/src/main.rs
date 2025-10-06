// Enforce Rust safety rule per AIBD-RG Section 7.1
#![forbid(unsafe_code)]

use std::{
    process,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::Duration,
};

use signal_hook::{consts::TERM_SIGNALS, iterator::Signals};

fn main() {
    println!("Starting dataplane...");
    println!("Dataplane service started successfully.");

    // Setup graceful shutdown signal handling.
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    // Spawn a thread to listen for termination signals.
    thread::spawn(move || {
        let mut signals = Signals::new(TERM_SIGNALS).expect("Failed to set up signal handler");
        if let Some(_sig) = signals.forever().next() {
            println!("Received termination signal, shutting down dataplane...");
            r.store(false, Ordering::SeqCst);
        }
    });

    // Main runtime loop: keep service alive until signal received.
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(10));
    }

    println!("Dataplane stopped cleanly.");
    process::exit(0);
}
