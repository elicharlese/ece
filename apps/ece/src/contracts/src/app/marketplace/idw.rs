use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct IDWContract {
    owner: AccountId,
    agents: UnorderedMap<AccountId, IDWAgent>,
    templates: UnorderedMap<u64, Template>,
    next_template_id: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct IDWAgent {
    name: String,
    configuration: String,
    status: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Template {
    id: u64,
    name: String,
    price: Balance,
    owner_id: AccountId,
}

impl Default for IDWContract {
    fn default() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            agents: UnorderedMap::new(b"a"),
            templates: UnorderedMap::new(b"t"),
            next_template_id: 0,
        }
    }
}

#[near_bindgen]
impl IDWContract {
    pub fn new(owner: AccountId) -> Self {
        Self {
            owner,
            agents: UnorderedMap::new(b"a"),
            templates: UnorderedMap::new(b"t"),
            next_template_id: 0,
        }
    }

    pub fn configure_agent(&mut self, account_id: AccountId, name: String, configuration: String) {
        let agent = IDWAgent {
            name,
            configuration,
            status: "Configured".to_string(),
        };
        self.agents.insert(&account_id, &agent);
        env::log_str("Agent configured successfully");
    }

    pub fn test_agent(&mut self, account_id: AccountId) -> String {
        if let Some(mut agent) = self.agents.get(&account_id) {
            agent.status = "Tested".to_string();
            self.agents.insert(&account_id, &agent);
            env::log_str("Agent tested successfully");
            "Test successful".to_string()
        } else {
            "Agent not found".to_string()
        }
    }

    pub fn purchase_template(&mut self, name: String, price: Balance) -> u64 {
        let template_id = self.next_template_id;
        let owner_id = env::predecessor_account_id();
        let template = Template {
            id: template_id,
            name,
            price,
            owner_id,
        };
        self.templates.insert(&template_id, &template);
        self.next_template_id += 1;
        env::log_str("Template purchased successfully");
        template_id
    }

    pub fn upload_template(&mut self, template_id: u64, name: String, price: Balance) {
        let owner_id = env::predecessor_account_id();
        let template = Template {
            id: template_id,
            name,
            price,
            owner_id,
        };
        self.templates.insert(&template_id, &template);
        env::log_str("Template uploaded successfully");
    }

    pub fn get_agent(&self, account_id: AccountId) -> Option<IDWAgent> {
        self.agents.get(&account_id)
    }

    pub fn get_template(&self, template_id: u64) -> Option<Template> {
        self.templates.get(&template_id)
    }
}