extern crate pbr;

use std::io::{stdout, Write};
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

    scan_all_ports(host, sender).await;

    let mut stdout = stdout();
    let mut open_addresses = vec![];
    while let Some(r) = receiver.recv().await {
        progress_bar.inc();

        stdout.flush().unwrap();
        if r.open {
            open_addresses.push(r.address.clone());
            println!(" OPEN: {}", r.address);
            stdout.flush().unwrap();
        }
    }

    progress_bar.finish_print("Scanning is finished!");

    println!();
    println!("Open ports are:");
    open_addresses.iter().for_each(|open_port| {
       println!("{}", open_port);
    });
}
