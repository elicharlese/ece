// Define the cargo.toml dependency
// [dependencies]
// near-sdk = "3.1.0" 

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, Promise};

// Define the data structure of the contract.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BitCellular {
    owner: AccountId,
    data: LookupMap<String, String>, // This will map a key to data fetched
}

// Default implementation for the contract
impl Default for BitCellular {
    fn default() -> Self {
        Self {
            owner: env::signer_account_id(),
            data: LookupMap::new(b"d"),
        }
    }
}

// Implementation of the contract methods.
#[near_bindgen]
impl BitCellular {
    // Initialization method
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            owner,
            data: LookupMap::new(b"d"),
        }
    }

    // A method to fetch data from Link Oracle.
    pub fn fetch_data(&mut self, key: String, url: String) {
        // Placeholder for actual data fetching logic using Link Oracle.
        // For now, we simulate a fetch with a Promise. In reality, we would interact with the oracle here.
        let data = self.query_link_oracle(url);
        self.data.insert(&key, &data);
    }

    // A helper function to simulate interacting with the Link Oracle.
    // In reality, this would contain asynchronous logic to call an external oracle and get the data.
    fn query_link_oracle(&self, url: String) -> String {
        // Simulate a fetch from the oracle.
        // In practice, this might include a Promise, callback, etc.
        format!("Data from {}", url) // Replace with real oracle integration.
    }

    // A method to retrieve stored data
    pub fn get_data(&self, key: String) -> Option<String> {
        self.data.get(&key)
    }

    // Only the owner can call this method.
    pub fn set_data(&mut self, key: String, value: String) {
        assert_eq!(self.owner, env::signer_account_id(), "Only the owner can set data");
        self.data.insert(&key, &value);
    }
}