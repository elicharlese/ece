use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SubscriptionContract {
    owner: AccountId,
    subscriptions: UnorderedMap<AccountId, Subscription>,
    token_balance: UnorderedMap<AccountId, Balance>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Subscription {
    tier: String,
    perks: Vec<String>,
    token_amount: Balance,
}

impl Default for SubscriptionContract {
    fn default() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            subscriptions: UnorderedMap::new(b"s"),
            token_balance: UnorderedMap::new(b"t"),
        }
    }
}

#[near_bindgen]
impl SubscriptionContract {
    pub fn subscribe(&mut self, account_id: AccountId, tier: String) {
        let (token_amount, perks) = match tier.as_str() {
            "Basic" => (100, vec!["Basic Perk 1".to_string(), "Basic Perk 2".to_string()]),
            "Standard" => (200, vec!["Standard Perk 1".to_string(), "Standard Perk 2".to_string()]),
            "Premium" => (500, vec!["Premium Perk 1".to_string(), "Premium Perk 2".to_string()]),
            _ => panic!("Invalid tier"),
        };

        let subscription = Subscription {
            tier,
            perks,
            token_amount,
        };

        self.subscriptions.insert(&account_id, &subscription);
        self.token_balance.insert(&account_id, &token_amount);
        env::log_str("Subscription successful");
    }

    pub fn purchase_tokens(&mut self, account_id: AccountId, amount: Balance) {
        let current_balance = self.token_balance.get(&account_id).unwrap_or(0);
        self.token_balance.insert(&account_id, &(current_balance + amount));
        env::log_str("Tokens purchased successfully");
    }

    pub fn get_subscription(&self, account_id: AccountId) -> Option<Subscription> {
        self.subscriptions.get(&account_id)
    }

    pub fn get_token_balance(&self, account_id: AccountId) -> Balance {
        self.token_balance.get(&account_id).unwrap_or(0)
    }
}