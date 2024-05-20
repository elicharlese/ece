// apps/ece/src/app/contracts/src/utils/utils.rs

// Utility functions can be added here

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
    // Dummy implementation, replace with actual logic
    1627845123
}

// Add more utility functions as needed