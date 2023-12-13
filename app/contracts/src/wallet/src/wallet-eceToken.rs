use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    balances: UnorderedMap<AccountId, Balance>,
    total_supply: Balance,
}

impl Default for ECE_Token {
    fn default() -> Self {
        Self {
            balances: UnorderedMap::new(b"b".to_vec()),
            total_supply: 0,
        }
    }
}

#[near_bindgen]
impl ECE_Token {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_supply(&self) -> Balance {
        self.total_supply
    }

    pub fn balance_of(&self, account_id: AccountId) -> Balance {
        self.balances.get(&account_id).unwrap_or(0)
    }

    pub fn transfer(&mut self, receiver_id: AccountId, amount: Balance) {
        let sender_id = env::predecessor_account_id();
        let sender_balance = self.balance_of(sender_id.clone());
        assert!(
            sender_balance >= amount,
            "Not enough balance to transfer"
        );

        self.balances.insert(&sender_id, &(sender_balance - amount));
        self.balances
            .insert(&receiver_id, &(self.balance_of(receiver_id.clone()) + amount));
    }
}
