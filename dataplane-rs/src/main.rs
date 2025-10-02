use dataplane_rs::features::forwarding::service::ForwardingService;

fn main() {
    let svc = ForwardingService::default();
    let _pipeline = svc
        .setup_pipeline("eth0", "eth1")
        .expect("failed to set up pipeline");

    println!("Forwarding service started");
}
