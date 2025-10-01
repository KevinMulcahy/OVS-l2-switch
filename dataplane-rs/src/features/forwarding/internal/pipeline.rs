// src/features/forwarding/internal/pipeline.rs
use crate::features::forwarding::types::NetIf;
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType, recv, send};
use std::io;

/// Minimal forwarder using raw Linux sockets
pub struct Forwarder {
    pub input: NetIf,
    pub output: NetIf,
}

impl Forwarder {
    pub fn new(input: NetIf, output: NetIf) -> Self {
        Self { input, output }
    }

    pub fn run(&self) -> io::Result<()> {
        // Open raw sockets (promiscuous mode)
        let in_fd = socket(AddressFamily::Packet, SockType::Raw, SockFlag::empty(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let out_fd = socket(AddressFamily::Packet, SockType::Raw, SockFlag::empty(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut buf = [0u8; 65536];

        loop {
            // Receive packet on input
            let n = recv(in_fd, &mut buf, nix::sys::socket::MsgFlags::empty())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Forward to output
            send(out_fd, &buf[..n], nix::sys::socket::MsgFlags::empty())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        // Ok(())  // unreachable, loop runs indefinitely
    }
}
