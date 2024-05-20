use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use near_sdk::collections::UnorderedMap;
use std::collections::HashMap;

// Define the structure of the contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ThePublic {
    data_storage: UnorderedMap<AccountId, String>,
    oracle_account: AccountId,
}

// Implementing Default trait for initializing the contract
impl Default for ThePublic {
    fn default() -> Self {
        Self {
            data_storage: UnorderedMap::new(b"d".to_vec()),
            oracle_account: env::current_account_id(),
        }
    }
}

// Define the Link Oracle-related methods
#[near_bindgen]
impl ThePublic {
    #[init]
    pub fn new(oracle_account: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            data_storage: UnorderedMap::new(b"d".to_vec()),
            oracle_account,
        }
    }

    // Function to request data from the Oracle
    pub fn request_data_from_oracle(&self, query: String) -> Promise {
        Promise::new(self.oracle_account.clone()).function_call(
            b"oracle_query".to_vec(),
            format!(r#"{{
                "query": "{}",
                "callback_url": "{}"
            }}"#, query, env::current_account_id()).into_bytes(),
            0,
            near_sdk::Gas(5_000_000_000_000),
        )
    }

    // Callback function to handle data received from the Oracle
    pub fn oracle_callback(&mut self, account_id: AccountId, data: String) -> String {
        self.data_storage.insert(&account_id, &data);
        data
    }

    // View method to retrieve stored data
    pub fn get_data(&self, sender: AccountId) -> Option<String> {
        self.data_storage.get(&sender)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let contract = ThePublic::default();
        assert_eq!(contract.data_storage.len(), 0);
    }

    #[test]
    fn test_data_storage() {
        let mut contract = ThePublic::default();
        contract.oracle_callback("test_account".to_string(), "test_data".to_string());
        let result = contract.get_data("test_account".to_string());
        assert_eq!(result, Some("test_data".to_string()));
    }
}