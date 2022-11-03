use std::env;
use tokio::sync::mpsc::Sender;
use tokio::net::TcpStream;

const DEFAULT_HOST: &str = "localhost";

#[derive(Debug, PartialEq, Eq)]
pub struct ScanResult {
    pub open: bool,
    pub address: String,
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
            let result = TcpStream::connect(address.clone()).await;
            let result = match result {
                Ok(_) => ScanResult { open: true, address: address.clone() },
                Err(_) => ScanResult { open: false, address: address.clone() },
            };
            match sender.send(result).await {
                Err(err) => {
                    println!("Error on sending result to the channel: {}", err);
                },
                _ => {}
            }
        });
    }
}

fn build_address(host: &String, port: &u16) -> String {
    format!("{host}:{port}")
}
