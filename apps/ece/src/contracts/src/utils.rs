// apps/ece/src/app/contracts/src/utils/utils.rs

// Utility functions for the application

/// Converts a string to title case
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Helper function to get the current timestamp
pub fn now() -> u64 {
    // Returns the current timestamp in seconds
    env::block_timestamp() / 1_000_000_000 // Convert nanoseconds to seconds
}

/// Validates an email address format
pub fn is_valid_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}

/// Validates a username to ensure it meets certain criteria
pub fn is_valid_username(username: &str) -> bool {
    !username.is_empty() && username.len() <= 30 // Example criteria: non-empty and max length of 30
}

/// Generates a random unique identifier (UUID)
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Converts a balance from one unit to another (e.g., from ECE to NEAR)
pub fn convert_balance(amount: u128, conversion_rate: f64) -> u128 {
    (amount as f64 * conversion_rate) as u128
}

// Add more utility functions as needed