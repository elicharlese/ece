mod utils;

// Implementation of Campaign Logic for Web3 Foundation Grant Funding
struct Web3Campaign {
    id: u64,
    name: String,
    description: String,
    target_amount: u128,
    current_amount: u128,
    start_date: u64,
    end_date: u64,
    owner: String,
}

impl Web3Campaign {
    // Initializes a new campaign with the given details
    fn new(id: u64, name: String, description: String, target_amount: u128, current_amount: u128, start_date: u64, end_date: u64, owner: String) -> Self {
        Web3Campaign {
            id,
            name,
            description,
            target_amount,
            current_amount,
            start_date,
            end_date,
            owner,
        }
    }

    // Function to support funding logic
    fn fund(&mut self, amount: u128) {
        if self.end_date > now() {
            self.current_amount += amount;
        }
    }
}

// Implementation of Gitcoin Grant Funding
struct GitcoinCampaign {
    id: u64,
    name: String,
    description: String,
    target_amount: u128,
    current_amount: u128,
    start_date: u64,
    end_date: u64,
    owner: String,
}

impl GitcoinCampaign {
    // Initializes a new campaign with the given details
    fn new(id: u64, name: String, description: String, target_amount: u128, current_amount: u128, start_date: u64, end_date: u64, owner: String) -> Self {
        GitcoinCampaign {
            id,
            name,
            description,
            target_amount,
            current_amount,
            start_date,
            end_date,
            owner,
        }
    }

    // Function to support funding logic
    fn fund(&mut self, amount: u128) {
        if self.end_date > now() {
            self.current_amount += amount;
        }
    }
}

// Smart contract for Startups
struct StartupCampaign {
    id: u64,
    name: String,
    description: String,
    target_amount: u128,
    current_amount: u128,
    start_date: u64,
    end_date: u64,
    owner: String,
}

impl StartupCampaign {
    // Initializes a new campaign with the given details
    fn new(id: u64, name: String, description: String, target_amount: u128, current_amount: u128, start_date: u64, end_date: u64, owner: String) -> Self {
        StartupCampaign {
            id,
            name,
            description,
            target_amount,
            current_amount,
            start_date,
            end_date,
            owner,
        }
    }

    // Function to support funding logic
    fn fund(&mut self, amount: u128) {
        if self.end_date > now() {
            self.current_amount += amount;
        }
    }
}

// Hypothetical function to get the current time
fn now() -> u64 {
    // Placeholder implementation; this should return the current time in the same format as `start_date` and `end_date`
    1627849267 // Example timestamp
}