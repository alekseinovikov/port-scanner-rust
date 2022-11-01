use std::env;
use tokio::{net::TcpSocket, sync::mpsc::Sender};

const DEFAULT_HOST: &str = "localhost";

#[derive(Debug, PartialEq, Eq)]
pub struct ScanResult {
    port: u8,
    open: bool
}

pub fn get_host() -> String{
    let args: Vec<String> = env::args().collect();
    return args.get(1).unwrap_or(&DEFAULT_HOST.to_owned()).to_owned();
}

pub async fn scan_all_ports(host: String, sender: Sender<ScanResult>) {
    for i in 0..=u8::MAX {
        let sender = sender.clone();
        let address = build_address(&host, &i);
        tokio::spawn(async move {
            let addr = address.parse().unwrap();
            let socket = TcpSocket::new_v4().unwrap();
            let result = socket.connect(addr).await;
            let result = match result {
                Ok(_) => ScanResult { port: i, open: true },
                Err(_) => ScanResult { port: i, open: false },
            };
            sender.send(result);
        });
    }
}

fn build_address(host: &String, port: &u8) -> String {
    format!("{host}:{port}")
}
