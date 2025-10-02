use crate::features::forwarding::types::NetIf;
use anyhow::Result;
use nix::sys::socket::{recv, send, socket, AddressFamily, MsgFlags, SockFlag, SockType};
use std::os::unix::io::AsRawFd; // ✅ needed to convert OwnedFd → RawFd

pub struct Pipeline {
    pub input: NetIf,
    pub output: NetIf,
}

impl Pipeline {
    pub fn new(input: NetIf, output: NetIf) -> Self {
        Self { input, output }
    }

    pub fn run(&self) -> Result<()> {
        // Example buffer for forwarding packets
        let mut buf = [0u8; 1500];

        // Open sockets (example: raw packet sockets for input/output)
        let in_fd = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlag::empty(),
            None,
        )?;
        let out_fd = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlag::empty(),
            None,
        )?;

        loop {
            // ✅ Use `.as_raw_fd()` to convert OwnedFd → RawFd (i32)
            let n = recv(in_fd.as_raw_fd(), &mut buf, MsgFlags::empty())?;

            send(out_fd.as_raw_fd(), &buf[..n], MsgFlags::empty())?;
        }
    }
}
