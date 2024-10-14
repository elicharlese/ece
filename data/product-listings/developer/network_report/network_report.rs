// network_report.rs

use rustscan_cli::Scanner;
use prettytable::{Table, row};
use std::net::IpAddr;
use std::str::FromStr;
use reqwest::Error;
use serde::Deserialize;
use tokio;

#[derive(Deserialize)]
struct Vulnerability {
    id: String,
    description: String,
    severity: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let target_ip = "192.168.1.1"; // Replace with your target IP
    let ports = "1-65535";
    let args = vec![
        "-b".to_string(), "1500".to_string(),
        "-a".to_string(), target_ip.to_string(),
        "-p".to_string(), ports.to_string(),
    ];

    // Port scanning
    let scanner = Scanner::parse(args).unwrap();
    let scan_result = scanner.scan().unwrap();

    // Vulnerability detection
    let response = reqwest::get(format!("https://api.mock.com/vulnerabilities?ip={}", target_ip)) // Replace with actual API URL
        .await?
        .json::<Vec<Vulnerability>>()
        .await?;

    // Combining results
    let mut table = Table::new();
    table.add_row(row!["Type", "ID/Port", "Description/Status", "Severity"]);

    // Add port scan results
    for port in scan_result {
        table.add_row(row!["Port", port.port, if port.status { "Open" } else { "Closed" }, ""]);
    }

    // Add vulnerability results
    for vuln in response {
        table.add_row(row!["Vulnerability", vuln.id, vuln.description, vuln.severity]);
    }

    table.printstd();
    Ok(())
}