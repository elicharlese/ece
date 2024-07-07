use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise, Gas};
use near_sdk::collections::UnorderedMap;
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ThePublic {
    data_storage: UnorderedMap<AccountId, String>,
    posts: UnorderedMap<u64, Post>,
    topics: UnorderedMap<u64, Topic>,
    users: HashMap<AccountId, User>,
    network_connections: HashMap<AccountId, String>,
    oracle_account: AccountId,
    moderators: Vec<AccountId>,
    post_count: u64,
    topic_count: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Post {
    account_id: AccountId,
    content: String,
    timestamp: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Topic {
    account_id: AccountId,
    title: String,
    timestamp: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    account_id: AccountId,
    username: String,
    connected_device: Option<String>,
    privacy_settings: PrivacySettings,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct PrivacySettings {
    show_posts: bool,
    show_topics: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            show_posts: true,
            show_topics: true,
        }
    }
}

impl Default for ThePublic {
    fn default() -> Self {
        Self {
            data_storage: UnorderedMap::new(b"d".to_vec()),
            posts: UnorderedMap::new(b"p".to_vec()),
            topics: UnorderedMap::new(b"t".to_vec()),
            users: HashMap::new(),
            network_connections: HashMap::new(),
            oracle_account: env::current_account_id(),
            moderators: vec![env::current_account_id()],
            post_count: 0,
            topic_count: 0,
        }
    }
}

#[near_bindgen]
impl ThePublic {
    #[init]
    pub fn new(oracle_account: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            data_storage: UnorderedMap::new(b"d".to_vec()),
            posts: UnorderedMap::new(b"p".to_vec()),
            topics: UnorderedMap::new(b"t".to_vec()),
            users: HashMap::new(),
            network_connections: HashMap::new(),
            oracle_account,
            moderators: vec![env::current_account_id()],
            post_count: 0,
            topic_count: 0,
        }
    }

    pub fn add_post(&mut self, account_id: AccountId, content: String) {
        let post = Post {
            account_id: account_id.clone(),
            content,
            timestamp: env::block_timestamp(),
        };
        self.posts.insert(&self.post_count, &post);
        self.post_count += 1;
        // TODO: Add integration with external smart contract for add_post
    }

    pub fn add_topic(&mut self, account_id: AccountId, title: String) {
        let topic = Topic {
            account_id: account_id.clone(),
            title,
            timestamp: env::block_timestamp(),
        };
        self.topics.insert(&self.topic_count, &topic);
        self.topic_count += 1;
        // TODO: Add integration with external smart contract for add_topic
    }

    pub fn moderate_post(&mut self, post_id: u64) {
        let caller = env::predecessor_account_id();
        assert!(self.moderators.contains(&caller), "Caller is not a moderator");
        self.posts.remove(&post_id);
        // TODO: Add integration with external smart contract for moderate_post
    }

    pub fn moderate_topic(&mut self, topic_id: u64) {
        let caller = env::predecessor_account_id();
        assert!(self.moderators.contains(&caller), "Caller is not a moderator");
        self.topics.remove(&topic_id);
        // TODO: Add integration with external smart contract for moderate_topic
    }

    pub fn register_user(&mut self, account_id: AccountId, username: String) {
        let user = User {
            account_id: account_id.clone(),
            username,
            connected_device: None,
            privacy_settings: PrivacySettings::default(),
        };
        self.users.insert(account_id, user);
        // TODO: Add integration with external smart contract for register_user
    }

    pub fn update_privacy_settings(&mut self, account_id: AccountId, show_posts: bool, show_topics: bool) {
        let user = self.users.get_mut(&account_id).expect("User not found");
        user.privacy_settings = PrivacySettings {
            show_posts,
            show_topics,
        };
        // TODO: Add integration with external smart contract for update_privacy_settings
    }

    pub fn connect_to_network(&mut self, account_id: AccountId, network: String) {
        self.network_connections.insert(account_id, network);
        // TODO: Add integration with external smart contract for connect_to_network
    }

    pub fn disconnect_from_network(&mut self, account_id: AccountId) {
        self.network_connections.remove(&account_id);
        // TODO: Add integration with external smart contract for disconnect_from_network
    }

    pub fn request_data_from_oracle(&self, query: String) -> Promise {
        Promise::new(self.oracle_account.clone()).function_call(
            b"oracle_query".to_vec(),
            format!(r#"{{
                "query": "{}",
                "callback_url": "{}"
            }}"#, query, env::current_account_id()).into_bytes(),
            0,
            Gas(5_000_000_000_000),
        )
        // TODO: Add integration with external smart contract for request_data_from_oracle
    }

    pub fn oracle_callback(&mut self, account_id: AccountId, data: String) -> String {
        self.data_storage.insert(&account_id, &data);
        data
        // TODO: Add integration with external smart contract for oracle_callback
    }

    pub fn get_data(&self, account_id: AccountId) -> Option<String> {
        self.data_storage.get(&account_id)
        // TODO: Add integration with external smart contract for get_data
    }

    pub fn get_posts(&self, from: u64, to: u64) -> Vec<Post> {
        (from..to)
            .filter_map(|id| self.posts.get(&id))
            .collect()
        // TODO: Add integration with external smart contract for get_posts
    }

    pub fn get_topics(&self, from: u64, to: u64) -> Vec<Topic> {
        (from..to)
            .filter_map(|id| self.topics.get(&id))
            .collect()
        // TODO: Add integration with external smart contract for get_topics
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proxy {
    logic_contract: AccountId,
}

#[near_bindgen]
impl Proxy {
    #[init]
    pub fn new(logic_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Proxy is already initialized");
        Self { logic_contract }
    }

    #[payable]
    pub fn proxy_call(&self, method_name: String, args: Vec<u8>, attached_deposit: u128, gas: Gas) -> Promise {
        Promise::new(self.logic_contract.clone()).function_call(
            method_name.into_bytes(),
            args,
            attached_deposit,
            gas,
        )
        // TODO: Add integration with external smart contract for proxy_call
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: predecessor_account_id.clone(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_initialization() {
        let context = get_context("bob_near".to_string());
        testing_env!(context);
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

    #[test]
    fn test_user_registration() {
        let mut contract = ThePublic::default();
        contract.register_user("test_account".to_string(), "test_user".to_string());
        let user = contract.users.get("test_account").unwrap();
        assert_eq!(user.username, "test_user".to_string());
    }

    #[test]
    fn test_connect_to_network() {
        let mut contract = ThePublic::default();
        contract.register_user("test_account".to_string(), "test_user".to_string());
        contract.connect_to_network("test_account".to_string(), "test_network".to_string());
        let network = contract.network_connections.get("test_account").unwrap();
        assert_eq!(network, "test_network");
    }

    #[test]
    fn test_proxy_initialization() {
        let context = get_context("bob_near".to_string());
        testing_env!(context);
        let proxy = Proxy::new("logic_contract".to_string());
        assert_eq!(proxy.logic_contract, "logic_contract".to_string());
    }

    #[test]
    fn test_data_storage_via_proxy() {
        let context = get_context("bob_near".to_string());
        testing_env!(context);
        let proxy = Proxy::new("logic_contract".to_string());

        let mut contract = ThePublic::default();
        contract.oracle_callback("test_account".to_string(), "test_data".to_string());
        // Assuming proxy_call is properly set to simulate the behavior of calling contract methods via proxy
        let result = contract.get_data("test_account".to_string());
        assert_eq!(result, Some("test_data".to_string()));
    }

    #[test]
    fn test_user_registration_via_proxy() {
        let context = get_context("bob_near".to_string());
        testing_env!(context);
        let proxy = Proxy::new("logic_contract".to_string());

        let mut contract = ThePublic::default();
        contract.register_user("test_account".to_string(), "test_user".to_string());
        // Assuming proxy_call is properly set to simulate the behavior of calling contract methods via proxy
        let user = contract.users.get("test_account").unwrap();
        assert_eq!(user.username, "test_user".to_string());
    }
}