use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;

// Assuming PerformanceMetrics and other dependencies are defined elsewhere
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PerformanceMetrics {
    // fields representing performance metrics
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OrganelleHealth {
    status: bool,
    last_check: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct HealthStatus {
    organelles: HashMap<String, OrganelleHealth>,
    alerts: Vec<String>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct HistoricalHeartbeat {
    timestamps: Vec<u64>,
    heartbeats: Vec<bool>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BitCellular {
    owner: AccountId,
    data: LookupMap<String, String>,
    performance_metrics: LookupMap<AccountId, PerformanceMetrics>,
    health_status: LookupMap<AccountId, HealthStatus>,
    historical_heartbeat: LookupMap<AccountId, HistoricalHeartbeat>, // new field to store historical heartbeat data
}

impl Default for BitCellular {
    fn default() -> Self {
        Self {
            owner: env::signer_account_id(),
            data: LookupMap::new(b"d"),
            performance_metrics: LookupMap::new(b"m"),
            health_status: LookupMap::new(b"h"),
            historical_heartbeat: LookupMap::new(b"hh"), // initialize the new field
        }
    }
}

#[near_bindgen]
impl BitCellular {
    // Function to update historical heartbeat
    pub fn update_heartbeat(&mut self, bitcell_id: AccountId, heartbeat: bool, timestamp: u64) {
        let mut historical_data = self.historical_heartbeat.get(&bitcell_id).unwrap_or_else(|| HistoricalHeartbeat {
            timestamps: Vec::new(),
            heartbeats: Vec::new(),
        });

        historical_data.timestamps.push(timestamp);
        historical_data.heartbeats.push(heartbeat);

        self.historical_heartbeat.insert(&bitcell_id, &historical_data);
    }

    // Function to calculate mean and standard deviation
    fn calculate_mean_and_std_dev(&self, bitcell_id: AccountId) -> (f64, f64) {
        if let Some(hist_data) = self.historical_heartbeat.get(&bitcell_id) {
            let heartbeat_values: Vec<u64> = hist_data.heartbeats.iter().map(|&hb| hb as u64).collect();
            let mean: f64 = heartbeat_values.iter().sum::<u64>() as f64 / heartbeat_values.len() as f64;

            let variance: f64 = heartbeat_values.iter().map(|&value| {
                let diff = mean - value as f64;
                diff * diff
            }).sum::<f64>() / heartbeat_values.len() as f64;

            let std_dev = variance.sqrt();
            (mean, std_dev)
        } else {
            (0.0, 0.0)
        }
    }

    // Function to check Bollinger bands
    pub fn check_bollinger_bands(&self, bitcell_id: AccountId, benchmark: f64) -> bool {
        let (mean, std_dev) = self.calculate_mean_and_std_dev(bitcell_id);
        let upper_band = mean + 2.0 * std_dev;
        let lower_band = mean - 2.0 * std_dev;

        benchmark < lower_band || benchmark > upper_band
    }

    pub fn update_organelle_health(&mut self, bitcell_id: AccountId, organelle: String, health_status: bool, timestamp: u64) {
        let mut health_data = self.health_status.get(&bitcell_id).unwrap_or_else(|| HealthStatus {
            organelles: HashMap::new(),
            alerts: Vec::new()
        });

        let organelle_health = OrganelleHealth {
            status: health_status,
            last_check: timestamp,
        };

        health_data.organelles.insert(organelle.clone(), organelle_health);

        self.check_for_alerts(&bitcell_id, &organelle, &health_data);

        self.health_status.insert(&bitcell_id, &health_data);
    }

    fn check_for_alerts(&self, bitcell_id: &AccountId, organelle: &String, health_data: &HealthStatus) {
        // Example alert condition: organelle health status is false
        if !health_data.organelles[organelle].status {
            let alert_message = format!("Alert: Organelle {} in bitcell {} is down.", organelle, bitcell_id);
            self.send_alert(&alert_message);
        }
    }

    fn send_alert(&self, message: &str) {
        // Placeholder for sending notifications/alerts
        // Could be sending email, writing to a log, or any other type of alert
        env::log_str(message);
    }

    // TODO: Add function to connect to external Bitcell smart contract for fetching data

    // Example function signature:
    // pub fn fetch_external_data(&self, contract_address: AccountId, some_param: SomeType) -> SomeReturnType {
    //     // Code to connect and fetch data from the external contract
    // }
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::MockedBlockchain;
  use near_sdk::{testing_env, VMContext};

  fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
      VMContext {
          current_account_id: "alice.testnet".to_string(),
          signer_account_id: "bob.testnet".to_string(),
          signer_account_pk: vec![0, 1, 2],
          predecessor_account_id: "carol.testnet".to_string(),
          input,
          block_index: 0,
          block_timestamp: 0,
          account_balance: 0,
          account_locked_balance: 0,
          storage_usage: 0,
          attached_deposit: 0,
          prepaid_gas: 10u64.pow(18),
          random_seed: vec![0, 1, 2],
          is_view,
          output_data_receivers: vec![],
          epoch_height: 19,
      }
  }

  #[test]
  fn test_update_organelle_health() {
      let context = get_context(vec![], false);
      testing_env!(context);
      let mut contract = BitCellular::default();

      contract.update_organelle_health(
          "bob.testnet".parse().unwrap(),
          "mitochondria".to_string(),
          true,
          1617181723
      );

      let health_status = contract.health_status.get(&"bob.testnet".parse().unwrap()).unwrap();
      assert!(health_status.organelles.get("mitochondria").unwrap().status);
  }

  #[test]
  fn test_heartbeat_update_and_calculation() {
      let context = get_context(vec![], false);
      testing_env!(context);
      let mut contract = BitCellular::default();

      contract.update_heartbeat("bob.testnet".parse().unwrap(), true, 1617181723);
      contract.update_heartbeat("bob.testnet".parse().unwrap(), false, 1617181823);

      let (mean, std_dev) = contract.calculate_mean_and_std_dev("bob.testnet".parse().unwrap());

      assert!(mean > 0.0); // Update with appropriate assertions
      assert!(std_dev >= 0.0);
  }
}