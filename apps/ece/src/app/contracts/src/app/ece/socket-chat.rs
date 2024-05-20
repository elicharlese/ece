use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Messages,
    Notes,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SocketChat {
    owner: AccountId,
    participants: Vec<AccountId>,
    messages: Vec<Message>,
    notes: Vec<Note>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Message {
    sender: AccountId,
    content: String,
    timestamp: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Note {
    content: String,
    reference_url: Option<String>,
}

#[near_bindgen]
impl SocketChat {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            owner,
            participants: Vec::new(),
            messages: Vec::new(),
            notes: Vec::new(),
        }
    }

    // Start the SocketChat application
    pub fn start_socket_chat(&self) {
        env::log_str("SocketChat application started");
    }

    // Run a CDE on a blockchain (selected by the user)
    pub fn run_cde(&self, blockchain: String) {
        match blockchain.as_str() {
            "Ethereum" => env::log_str("Running CDE on Ethereum using Chainlink Oracle"),
            "Polkadot" => env::log_str("Running CDE on Polkadot using Chainlink Oracle"),
            _ => env::log_str("Unknown blockchain selected"),
        }
    }

    // Handle chat with other users
    pub fn handle_chat(&mut self, user: AccountId, message: String) {
        self.participants.push(user.clone());
        let new_message = Message {
            sender: user.clone(),
            content: message.clone(),
            timestamp: env::block_timestamp(),
        };
        self.messages.push(new_message);
        env::log_str(&format!("User {} sent message: {}", user, message));
    }

    // View chat inline with code
    pub fn view_chat(&self) -> Vec<Message> {
        self.messages.clone()
    }

    // Comment without code
    pub fn add_comment(&mut self, user: AccountId, comment: String) {
        self.handle_chat(user, comment);
    }

    // Tag other users
    pub fn tag_user(&mut self, tagger: AccountId, tagged_user: AccountId, message: String) {
        let tag_message = format!("@{}: {}", tagged_user, message);
        self.handle_chat(tagger, tag_message);
    }

    // Pull in notes
    pub fn add_note(&mut self, content: String, reference_url: Option<String>) {
        let new_note = Note {
            content,
            reference_url,
        };
        self.notes.push(new_note);
        env::log_str("Note added");
    }

    // Pull in other files and reference material from the web
    pub fn add_reference(&mut self, url: String, description: String) {
        self.add_note(description, Some(url));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        VMContextBuilder::new()
    }

    #[test]
    fn test_start_socket_chat() {
        let context = get_context();
        testing_env!(context.build());
        let contract = SocketChat::new("owner.testnet".parse().unwrap());
        contract.start_socket_chat();
    }

    #[test]
    fn test_run_cde() {
        let context = get_context();
        testing_env!(context.build());
        let contract = SocketChat::new("owner.testnet".parse().unwrap());
        contract.run_cde("Ethereum".to_string());
    }

    #[test]
    fn test_handle_chat() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        contract.handle_chat("user1.testnet".parse().unwrap(), "Hello!".to_string());
    }

    #[test]
    fn test_add_comment() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        contract.add_comment("user1.testnet".parse().unwrap(), "A comment".to_string());
    }

    #[test]
    fn test_tag_user() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        contract.tag_user(
            "user1.testnet".parse().unwrap(),
            "user2.testnet".parse().unwrap(),
            "Check this out".to_string()
        );
    }

    #[test]
    fn test_add_note() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        contract.add_note(
            "This is a note".to_string(),
            Some("http://example.com".to_string())
        );
    }

    #[test]
    fn test_add_reference() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        contract.add_reference(
            "http://example.com".to_string(),
            "This is a reference".to_string()
        );
    }
}