use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};
use serde::{Deserialize, Serialize};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Messages,
    Notes,
    Files,
    Comments,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SocketChat {
    owner: AccountId,
    participants: Vec<AccountId>,
    messages: Vec<Message>,
    notes: Vec<Note>,
    files: Vec<File>,
    comments: Vec<Comment>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
pub struct Message {
    sender: AccountId,
    content: String,
    timestamp: u64,
    file_id: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Note {
    content: String,
    reference_url: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct File {
    id: u64,
    name: String,
    content: String,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Comment {
    file_id: u64,
    line_number: u64,
    content: String,
    author: AccountId,
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
            files: Vec::new(),
            comments: Vec::new(),
        }
    }

    // Start the SocketChat application
    pub fn start_socket_chat(&self) {
        env::log_str("SocketChat application started");
    }

    // Handle chat with other users
    pub fn handle_chat(&mut self, user: AccountId, message: String, file_id: Option<u64>) {
        self.participants.push(user.clone());
        let new_message = Message {
            sender: user.clone(),
            content: message.clone(),
            timestamp: env::block_timestamp(),
            file_id,
        };
        self.messages.push(new_message);
        env::log_str(&format!("User {} sent message: {}", user, message));
    }

    // View chat inline with code
    pub fn view_chat(&self) -> Vec<Message> {
        self.messages.clone()
    }

    // Add a new file to the system
    pub fn add_file(&mut self, name: String, content: String) -> u64 {
        let file_id = self.files.len() as u64;
        self.files.push(File { id: file_id, name, content });
        env::log_str(&format!("File added with ID: {}", file_id));
        file_id
    }

    // Add a comment to a specific line in a file
    pub fn add_comment(&mut self, user: AccountId, file_id: u64, line_number: u64, content: String) {
        let new_comment = Comment { file_id, line_number, content, author: user };
        self.comments.push(new_comment);
        env::log_str("Comment added");
    }

    // View file along with its comments
    pub fn view_file_with_comments(&self, file_id: u64) -> Option<(File, Vec<Comment>)> {
        let file = self.files.iter().find(|f| f.id == file_id).cloned();
        let comments: Vec<Comment> = self.comments.iter().filter(|c| c.file_id == file_id).cloned().collect();
        if let Some(f) = file {
            Some((f, comments))
        } else {
            None
        }
    }

    // Add a note
    pub fn add_note(&mut self, content: String, reference_url: Option<String>) {
        let new_note = Note { content, reference_url };
        self.notes.push(new_note);
        env::log_str("Note added");
    }

    // Add a reference URL
    pub fn add_reference(&mut self, url: String, description: String) {
        self.add_note(description, Some(url));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
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
    fn test_handle_chat_with_file() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        let file_id = contract.add_file("test.txt".to_string(), "Some content".to_string());
        contract.handle_chat("user1.testnet".parse().unwrap(), "Check out the file", Some(file_id));
    }

    #[test]
    fn test_add_comment() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        let file_id = contract.add_file("test.txt".to_string(), "Some content".to_string());
        contract.add_comment("user1.testnet".parse().unwrap(), file_id, 1, "Nice explanation".to_string());
    }

    #[test]
    fn test_view_file_with_comments() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = SocketChat::new("owner.testnet".parse().unwrap());
        let file_id = contract.add_file("test.txt".to_string(), "Some content".to_string());
        contract.add_comment("user1.testnet".parse().unwrap(), file_id, 1, "Nice explanation".to_string());
        let result = contract.view_file_with_comments(file_id);
        assert!(result.is_some());
    }
}