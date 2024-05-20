use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NanoNodeSettings {
    version: String,
    strategy: String,
    strategy_file: String,
    network: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NanoNode {
    settings: LookupMap<AccountId, NanoNodeSettings>,
    terminal: LookupMap<AccountId, String>,
    output_log: LookupMap<AccountId, String>,
    statistics: LookupMap<AccountId, String>,
}

impl Default for NanoNode {
    fn default() -> Self {
        Self {
            settings: LookupMap::new(b"s"),
            terminal: LookupMap::new(b"t"),
            output_log: LookupMap::new(b"o"),
            statistics: LookupMap::new(b"a"),
        }
    }
}

#[near_bindgen]
impl NanoNode {
    // Set NanoNode settings
    pub fn set_settings(&mut self, account_id: AccountId, version: String, strategy: String, strategy_file: String, network: String) {
        let settings = NanoNodeSettings {
            version,
            strategy,
            strategy_file,
            network,
        };
        self.settings.insert(&account_id, &settings);
    }

    // Get NanoNode settings
    pub fn get_settings(&self, account_id: AccountId) -> Option<NanoNodeSettings> {
        self.settings.get(&account_id)
    }

    // Set NanoNode terminal info
    pub fn set_terminal(&mut self, account_id: AccountId, terminal: String) {
        self.terminal.insert(&account_id, &terminal);
    }

    // Get NanoNode terminal info
    pub fn get_terminal(&self, account_id: AccountId) -> Option<String> {
        self.terminal.get(&account_id)
    }

    // Set NanoNode output log
    pub fn set_output_log(&mut self, account_id: AccountId, output_log: String) {
        self.output_log.insert(&account_id, &output_log);
    }

    // Get NanoNode output log
    pub fn get_output_log(&self, account_id: AccountId) -> Option<String> {
        self.output_log.get(&account_id)
    }

    // Set NanoNode statistics
    pub fn set_statistics(&mut self, account_id: AccountId, statistics: String) {
        self.statistics.insert(&account_id, &statistics);
    }

    // Get NanoNode statistics
    pub fn get_statistics(&self, account_id: AccountId) -> Option<String> {
        self.statistics.get(&account_id)
    }
}