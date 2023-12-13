use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct MultisigWallet {
    owners: Vec<AccountId>,
    required_confirmations: u8,
}

impl MultisigWallet {
    pub fn new(owners: Vec<AccountId>, required_confirmations: u8) -> Self {
        Self {
            owners,
            required_confirmations, // called a quorum
        }
    }

    pub fn transfer(&self, recipient: AccountId, amount: Balance) {
        // Perform transfer logic here
    }

    pub fn add_owner(&mut self, owner: AccountId) {
        // Add owner logic here
    }

    pub fn remove_owner(&mut self, owner: AccountId) {
        // Remove owner logic here
    }

    pub fn change_required_confirmations(&mut self, required_confirmations: u8) {
        // Change required confirmations logic here
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;

    #[test]
    fn test_transfer() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);

        let mut contract = MultisigWallet::default();
        contract.transfer(accounts(2), 100);
        // Add assertions here
    }

    #[test]
    fn test_add_owner() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);

        let mut contract = MultisigWallet::default();
        contract.add_owner(accounts(2));
        // Add assertions here
    }

    #[test]
    fn test_remove_owner() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);

        let mut contract = MultisigWallet::default();
        contract.remove_owner(accounts(2));
        // Add assertions here
    }

    #[test]
    fn test_change_required_confirmations() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);

        let mut contract = MultisigWallet::default();
        contract.change_required_confirmations(2);
        // Add assertions here
    }
}
