// src/lib.rs

// Declare the modules 
pub mod contracts;
pub mod api;
pub mod utils;

// Re-exporting commonly used types and functions for easier access
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

pub use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::program::invoke;
pub use anchor_lang::solana_program::account_info::AccountInfo;
pub use anchor_lang::solana_program::entrypoint::ProgramResult;
pub use anchor_lang::solana_program::msg;
pub use anchor_lang::solana_program::system_instruction;

// Main application struct to manage the various components
#[derive(AnchorSerialize, AnchorDeserialize)]
#[account]
pub struct Application {
    pub wallet: Wallet,
    pub marketplace: Marketplace,
    pub crowdfunding_center: CrowdfundingCenter,
    pub the_public: ThePublic,
    pub bitcell: Bitcell,
    pub txn_tracker: TxnTracker,
    pub nano_node: NanoNode,
    pub socket_chat: SocketChat,
}

// Implementation of the Application struct
impl Application {
    #[init]
    pub fn new() -> Self {
        Self {
            wallet: Wallet::new(),
            marketplace: Marketplace::new(),
            crowdfunding_center: CrowdfundingCenter::new(),
            the_public: ThePublic::new(),
            bitcell: Bitcell::new(),
            txn_tracker: TxnTracker::new(),
            nano_node: NanoNode::new(),
            socket_chat: SocketChat::new(),
        }
    }

    /// Function to handle wallet interactions
    pub fn handle_wallet(&mut self) {
        self.wallet.process_transactions();
    }

    /// Function to list a product in the marketplace
    /// 
    /// # Arguments
    /// * `id` - The unique identifier for the product.
    /// * `owner_id` - The account ID of the product owner.
    /// * `metadata` - Metadata associated with the product.
    /// * `price` - The price of the product.
    pub fn list_product(&mut self, id: u64, owner_id: Pubkey, metadata: String, price: u64) {
        self.marketplace.list_product(id, owner_id, metadata, price);
    }

    /// Function to create a crowdfunding campaign
    /// 
    /// # Arguments
    /// * `id` - The unique identifier for the campaign.
    /// * `owner_id` - The account ID of the campaign owner.
    /// * `goal` - The funding goal for the campaign.
    pub fn create_campaign(&mut self, id: String, owner_id: Pubkey, goal: u64) {
        self.crowdfunding_center.create_campaign(id, owner_id, goal);
    }

    /// Function to register a user in ThePublic application
    /// 
    /// # Arguments
    /// * `account_id` - The account ID of the user.
    /// * `username` - The username of the user.
    pub fn register_user(&mut self, account_id: Pubkey, username: String) {
        self.the_public.register_user(account_id, username);
    }

    /// Function to track a transaction
    /// 
    /// # Arguments
    /// * `transaction_id` - The unique identifier for the transaction.
    /// * `transaction_data` - The data associated with the transaction.
    /// * `involved_address` - The account ID involved in the transaction.
    pub fn track_transaction(&mut self, transaction_id: String, transaction_data: String, involved_address: Pubkey) {
        self.txn_tracker.track_transaction(transaction_id, transaction_data, involved_address);
    }

    /// Function to start the socket chat application
    pub fn start_socket_chat(&self) {
        self.socket_chat.start_socket_chat();
    }
}