extern crate pbr;

use port_scanner_rust::get_host;
use port_scanner_rust::scan_all_ports;
use port_scanner_rust::ScanResult;
use tokio::sync::mpsc::channel;

use pbr::ProgressBar;

#[tokio::main]
async fn main() {
    let host = get_host();
    let (sender, mut receiver) = channel::<ScanResult>(u8::MAX as usize);
    let mut progress_bar = ProgressBar::new(u16::MAX as u64);
    progress_bar.format("╢▌▌░╟");


    scan_all_ports(host, sender).await;

    let mut open_ports = vec![];
    while let Some(r) = receiver.recv().await {
        progress_bar.inc();

        if r.open {
            open_ports.push(r.port);
            println!(" OPEN: {}", r.port);
        }
    }

    progress_bar.finish_print("Scanning is finished!");

    println!();
    println!("Open ports are:");
    open_ports.iter().for_each(|open_port| {
       println!("{}", open_port);
    });
}
