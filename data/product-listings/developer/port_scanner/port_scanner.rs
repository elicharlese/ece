// port_scanner.rs

use rustscan_cli::Scanner;
use prettytable::{Table, row, cell};
use std::net::IpAddr;
use std::str::FromStr;

fn main() {
    let target_ip = "192.168.1.1"; // Replace with your target IP
    let ports = "1-65535";
    let args = vec![
        "-b".to_string(), "1500".to_string(),
        "-a".to_string(), target_ip.to_string(),
        "-p".to_string(), ports.to_string(),
    ];

    let scanner = Scanner::parse(args).unwrap();
    let scan_result = scanner.scan().unwrap();

    let mut table = Table::new();
    table.add_row(row!["Port", "Status"]);

    for port in scan_result {
        table.add_row(row![port.port, if port.status { "Open" } else { "Closed" }]);
    }

    table.printstd();
}