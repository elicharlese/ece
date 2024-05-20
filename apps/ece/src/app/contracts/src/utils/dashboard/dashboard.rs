use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Smart Contract for providing a dashboard
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dashboard {
    title: String,
    widgets: Vec<Widget>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Widget {
    name: String,
    value: String,
}

impl Dashboard {
    pub fn new(title: String) -> Self {
        Dashboard {
            title,
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, name: String, value: String) {
        self.widgets.push(Widget { name, value });
    }

    pub fn add_moving_average_widget(&mut self, name: String, data: &Vec<f64>, period: usize) {
        let moving_average = StatisticalAnalysis::moving_average(data, period);
        self.add_widget(name, moving_average.to_string());
    }
}

// Smart Contract for statistical analysis
pub struct StatisticalAnalysis {}

impl StatisticalAnalysis {
    pub fn mean(data: &Vec<f64>) -> f64 {
        let sum: f64 = data.iter().sum();
        sum / data.len() as f64
    }

    pub fn median(data: &mut Vec<f64>) -> f64 {
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = data.len() / 2;
        if data.len() % 2 == 0 {
            (data[mid - 1] + data[mid]) / 2.0
        } else {
            data[mid]
        }
    }

    pub fn variance(data: &Vec<f64>) -> f64 {
        let mean = Self::mean(data);
        let sum_squared_diff: f64 = data.iter().map(|value| (value - mean).powi(2)).sum();
        sum_squared_diff / data.len() as f64
    }

    pub fn moving_average(data: &Vec<f64>, period: usize) -> f64 {
        let n = data.len();
        if n < period {
            return data.iter().sum::<f64>() / n as f64;
        }
        let slice = &data[n - period..];
        slice.iter().sum::<f64>() / period as f64
    }
}

// Smart Contract for blockchain analytics
pub struct BlockchainAnalytics {}

impl BlockchainAnalytics {
    pub fn count_transactions_by_user(transactions: &Vec<Transaction>, user: &str) -> usize {
        transactions.iter().filter(|tx| tx.user == user).count()
    }

    pub fn total_value_by_user(transactions: &Vec<Transaction>, user: &str) -> f64 {
        transactions.iter().filter(|tx| tx.user == user).map(|tx| tx.value).sum()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    user: String,
    value: f64,
    timestamp: u64,
}