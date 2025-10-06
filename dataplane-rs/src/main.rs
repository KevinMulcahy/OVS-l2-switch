// Enforce Rust safety rule per AIBD-RG Section 7.1
#![forbid(unsafe_code)]

use std::{
    io::Write,
    net::TcpListener,
    process,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::Duration,
};

use signal_hook::{consts::TERM_SIGNALS, iterator::Signals};

fn main() {
    println!("Starting dataplane...");

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    // Graceful shutdown handler
    thread::spawn(move || {
        let mut signals = Signals::new(TERM_SIGNALS).expect("Failed to set up signal handler");
        if let Some(_sig) = signals.forever().next() {
            println!("Received termination signal, shutting down dataplane...");
            r.store(false, Ordering::SeqCst);
        }
    });

    // Simple TCP health server for Docker healthcheck
    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind healthcheck port");
        println!("Dataplane healthcheck server listening on port 8080");
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let _ = stream.write_all(b"OK\n");
            }
        }
    });

    println!("Dataplane service started successfully.");

    // Main loop
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(10));
    }

    println!("Dataplane stopped cleanly.");
    process::exit(0);
}
