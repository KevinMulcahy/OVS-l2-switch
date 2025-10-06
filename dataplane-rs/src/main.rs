// Enforce Rust safety rule per AIBD-RG Section 7.1
#![forbid(unsafe_code)]

use std::{
    io::{Read, Write},
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

    // Graceful shutdown
    thread::spawn(move || {
        let mut signals = Signals::new(TERM_SIGNALS).expect("Failed to set up signal handler");
        if let Some(_sig) = signals.forever().next() {
            println!("Received termination signal, shutting down dataplane...");
            r.store(false, Ordering::SeqCst);
        }
    });

    // HTTP health server
    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind healthcheck port");
        println!("Dataplane HTTP healthcheck server listening on port 8080");

        for mut stream in listener.incoming().flatten() {
            let mut buffer = [0; 512];
            let _ = stream.read(&mut buffer);
            let response = concat!(
                "HTTP/1.1 200 OK\r\n",
                "Content-Type: text/plain\r\n",
                "Content-Length: 3\r\n",
                "Connection: close\r\n",
                "\r\n",
                "OK\n"
            );
            let _ = stream.write_all(response.as_bytes());
        }
    });

    println!("Dataplane service started successfully.");

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(10));
    }

    println!("Dataplane stopped cleanly.");
    process::exit(0);
}
