use std::env;
use tokio::sync::mpsc::Sender;
use tokio::net::TcpStream;

const DEFAULT_HOST: &str = "localhost";

#[derive(Debug, PartialEq, Eq)]
pub struct ScanResult {
    pub port: u16,
    pub open: bool
}

pub fn get_host() -> String{
    let args: Vec<String> = env::args().collect();
    return args.get(1).unwrap_or(&DEFAULT_HOST.to_owned()).to_owned();
}

pub async fn scan_all_ports(host: String, sender: Sender<ScanResult>) {
    for i in 0..=u16::MAX {
        let address = build_address(&host, &i);
        let sender = sender.clone();

        tokio::spawn(async move {
            let result = TcpStream::connect(address).await;
            let result = match result {
                Ok(_) => ScanResult { port: i, open: true },
                Err(_) => ScanResult { port: i, open: false },
            };
            sender.send(result).await.unwrap();
        });
    }
}

fn build_address(host: &String, port: &u16) -> String {
    format!("{host}:{port}")
}
