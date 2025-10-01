use crate::features::forwarding::service::ForwardingService;

fn main() -> anyhow::Result<()> {
    // Forward packets from eth0 → eth1
    ForwardingService::start_forwarding("eth0", "eth1")?;
    Ok(())
}
