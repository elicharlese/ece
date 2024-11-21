// all necessary imports and structures combined here

// Smart Contract for Marketplace
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Marketplace {
    listings: UnorderedMap<AccountId, Item>,
    balances: UnorderedMap<AccountId, Balance>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Item {
    id: String,
    owner_id: AccountId,
    price: Balance,
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            listings: UnorderedMap::new(b"l".to_vec()),
            balances: UnorderedMap::new(b"b".to_vec()),
        }
    }
}

#[near_bindgen]
impl Marketplace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list_item(&mut self, id: String, price: Balance) {
        let owner_id = env::predecessor_account_id();
        let item = Item {
            id: id.clone(),
            owner_id: owner_id.clone(),
            price,
        };
        self.listings.insert(&AccountId::new_unchecked(id), &item);
        env::log(format!("Item {} listed by {}", id, owner_id).as_bytes());
    }

    pub fn purchase_item(&mut self, id: String) -> Promise {
        let buyer_id = env::predecessor_account_id();
        let item = self.listings.get(&AccountId::new_unchecked(id.clone())).expect("Item not found");
        let buyer_balance = self.balance_of(buyer_id.clone());

        assert!(buyer_balance >= item.price, "Not enough balance to purchase");

        self.balances.insert(&buyer_id, &(buyer_balance - item.price));
        self.balances.insert(&item.owner_id, &(self.balance_of(item.owner_id.clone()) + item.price));
        self.listings.remove(&AccountId::new_unchecked(id));

        env::log(format!("Item {} purchased by {}", id, buyer_id).as_bytes());

        Promise::new(buyer_id).transfer(item.price)
    }

    // ECE Token deposit method
    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::predecessor_account_id();
        let amount = env::attached_deposit();
        let balance = self.balance_of(account_id.clone());
        self.balances.insert(&account_id, &(balance + amount));
        env::log(format!("Deposited {} to {}", amount, account_id).as_bytes());
    }

    #[payable]
    pub fn withdraw(&mut self, amount: Balance) {
        let account_id = env::predecessor_account_id();
        let balance = self.balance_of(account_id.clone());
        assert!(balance >= amount, "Not enough balance to withdraw");
        self.balances.insert(&account_id, &(balance - amount));
        Promise::new(account_id.clone()).transfer(amount);
        env::log(format!("Withdrew {} from {}", amount, account_id).as_bytes());
    }

    pub fn balance_of(&self, account_id: AccountId) -> Balance {
        self.balances.get(&account_id).unwrap_or(0)
    }

    // Provide information on the ECE token and its current balance, if any.
    // Also information about the ECE token's price (slippage) and the current market cap.
    pub fn get_info(&self) -> String {
        let token_info = env::token_info();
        let token_price = token_info.price.unwrap_or(0);
        let token_market_cap = token_info.market_cap.unwrap_or(0);

        let token_balance = self.balance_of(AccountId::new_unchecked(token_info.account_id));

        format!(
            "ECE Token: {}\nPrice: {}\nMarket Cap: {}",
            token_info.symbol, token_price, token_market_cap
        )
        format!(
            "ECE Balance: {}\nBalance: {}",
            token_info.balance, self.balance_of
        )
    }
}

// Smart Contract for Campaign
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Campaign {
    id: String,
    owner_id: AccountId,
    goal: Balance,
    collected: Balance,
}

impl Default for Campaign {
    fn default() -> Self {
        panic!("Campaign should be initialized with specific parameters");
    }
}

#[near_bindgen]
impl Campaign {
    pub fn new(id: String, owner_id: AccountId, goal: Balance) -> Self {
        Self {
            id,
            owner_id,
            goal,
            collected: 0,
        }
    }

    #[payable]
    pub fn donate(&mut self) {
        let donor_id = env::predecessor_account_id();
        let amount = env::attached_deposit();
        self.collected += amount;
        env::log(format!("{} donated {} to campaign {}", donor_id, amount, self.id).as_bytes());
    }

    pub fn withdraw(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only the owner can withdraw funds");
        assert!(self.collected > 0, "No funds to withdraw");
        let amount = self.collected;
        self.collected = 0;
        Promise::new(self.owner_id.clone()).transfer(amount);
        env::log(format!("Withdrawn {} from campaign {}", amount, self.id).as_bytes());
    }

    pub fn get_campaign_info(&self) -> String {
        format!(
            "Campaign: {}\nOwner: {}\nGoal: {}\nCollected: {}",
            self.id, self.owner_id, self.goal, self.collected
        )
    }
}

// ECE Token Contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ECE_Token {
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

// Multisend-wallet Contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MultisendWallet {
    owners: Vec<AccountId>,
    required_confirmations: u8,
    transactions: UnorderedMap<u64, Transaction>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Transaction {
    recipient: AccountId,
    amount: Balance,
    confirmations: u8,
}

impl Default for MultisendWallet {
    fn default() -> Self {
        Self {
            owners: Vec::new(),
            required_confirmations: 0,
            transactions: UnorderedMap::new(b"t".to_vec()),
        }
    }
}

#[near_bindgen]
impl MultisendWallet {
    pub fn new(owners: Vec<AccountId>, required_confirmations: u8) -> Self {
        Self {
            owners,
            required_confirmations,
            transactions: UnorderedMap::new(b"t".to_vec()),
        }
    }

    pub fn submit_transaction(&mut self, recipient: AccountId, amount: Balance) {
        // Implementation details
    }

    pub fn confirm_transaction(&mut self, transaction_id: u64) {
        // Implementation details
    }

    pub fn add_owner(&mut self, owner: AccountId) {
        self.owners.push(owner.clone());
        env::log(format!("Added owner {}", owner).as_bytes());
    }

    pub fn remove_owner(&mut self, owner: AccountId) {
        self.owners.retain(|a| a != &owner);
        env::log(format!("Removed owner {}", owner).as_bytes());
    }

    pub fn change_required_confirmations(&mut self, required_confirmations: u8) {
        self.required_confirmations = required_confirmations;
        env::log(format!("Required confirmations changed to {}", required_confirmations).as_bytes());
    }
}

// Wallet Factory Contract
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct WalletFactory {
    multisig_wallets: UnorderedMap<AccountId, AccountId>,  // Store mapping from user to their wallet
}

#[near_bindgen]
impl WalletFactory {
    pub fn create_multisig_wallet(&mut self, owners: Vec<AccountId>, required_confirmations: u8) -> AccountId {
        let caller = env::predecessor_account_id();
        let wallet_id = AccountId::new_unchecked(format!("wallet_{}", caller));

        // Deployment logic here

        self.multisig_wallets.insert(&caller, &wallet_id);
        wallet_id
    }
}

// Wallet Oracle Contract
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct WalletOracle {
    exchange_rates: UnorderedMap<AccountId, f64>,
}

#[near_bindgen]
impl WalletOracle {
    pub fn