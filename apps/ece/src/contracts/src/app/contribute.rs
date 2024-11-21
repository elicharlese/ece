mod utils;

// Structure to hold information about your startups
struct MyStartup {
    id: u64,
    name: String,
    description: String,
    target_amount: u128,
    current_amount: u128,
    start_date: u64,
    end_date: u64,
    owner: String,
}

impl MyStartup {
    fn new(id: u64, name: String, description: String, target_amount: u128, current_amount: u128, start_date: u64, end_date: u64, owner: String) -> Self {
        MyStartup {
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

    fn fund(&mut self, amount: u128) {
        if self.end_date > now() {
            self.current_amount += amount;
        }
    }
}

// Structure to hold information about startups you sponsor
struct SponsoredStartup {
    id: u64,
    name: String,
    description: String,
    target_amount: u128,
    current_amount: u128,
    start_date: u64,
    end_date: u64,
    sponsor: String,
}

impl SponsoredStartup {
    fn new(id: u64, name: String, description: String, target_amount: u128, current_amount: u128, start_date: u64, end_date: u64, sponsor: String) -> Self {
        SponsoredStartup {
            id,
            name,
            description,
            target_amount,
            current_amount,
            start_date,
            end_date,
            sponsor,
        }
    }

    fn fund(&mut self, amount: u128) {
        if self.end_date > now() {
            self.current_amount += amount;
        }
    }
}

// Existing FreelanceRequest structure can be used for freelance requests
// No changes needed for FreelanceRequest

// Hypothetical function to get the current time
fn now() -> u64 {
    Utc::now().timestamp() as u64
}

// New section for Freelance Quote Requests
struct FreelanceRequest {
    user_id: u64,
    project_name: String,
    project_description: String,
    budget: u128,
    deadline: u64,
}

impl FreelanceRequest {
    // Initializes a new freelance request
    fn new(user_id: u64, project_name: String, project_description: String, budget: u128, deadline: u64) -> Self {
        FreelanceRequest {
            user_id,
            project_name,
            project_description,
            budget,
            deadline,
        }
    }

    // Function to generate a quote
    fn generate_quote(&self) -> String {
        // Placeholder logic for generating a quote
        format!(
            "Quote for project '{}': Estimated cost is {} with a deadline of {}.",
            self.project_name, self.budget, self.deadline
        )
    }
}