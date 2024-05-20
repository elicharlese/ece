use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Startup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub goal_amount: f64,
    pub current_amount: f64,
    pub creator: String,
    pub backers: Vec<Backer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backer {
    pub name: String,
    pub amount: f64,
}

impl Startup {
    pub fn new(
        id: String,
        name: String,
        description: String,
        goal_amount: f64,
        creator: String,
    ) -> Self {
        Startup {
            id,
            name,
            description,
            goal_amount,
            current_amount: 0.0,
            creator,
            backers: vec![],
        }
    }

    pub fn add_backer(&mut self, backer: Backer) {
        self.current_amount += backer.amount;
        self.backers.push(backer);
    }

    pub fn is_funded(&self) -> bool {
        self.current_amount >= self.goal_amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup_creation() {
        let startup = Startup::new(
            "1".to_string(),
            "Innovative Tech".to_string(),
            "A new tech startup".to_string(),
            100000.0,
            "Alice".to_string(),
        );
        assert_eq!(startup.name, "Innovative Tech");
        assert_eq!(startup.creator, "Alice");
        assert_eq!(startup.goal_amount, 100000.0);
        assert_eq!(startup.current_amount, 0.0);
        assert!(startup.backers.is_empty());
    }

    #[test]
    fn test_add_backer() {
        let mut startup = Startup::new(
            "1".to_string(),
            "Innovative Tech".to_string(),
            "A new tech startup".to_string(),
            100000.0,
            "Alice".to_string(),
        );

        let backer = Backer {
            name: "Bob".to_string(),
            amount: 5000.0,
        };

        startup.add_backer(backer);
        assert_eq!(startup.current_amount, 5000.0);
        assert_eq!(startup.backers.len(), 1);
        assert_eq!(startup.backers[0].name, "Bob");
    }

    #[test]
    fn test_is_funded() {
        let mut startup = Startup::new(
            "1".to_string(),
            "Innovative Tech".to_string(),
            "A new tech startup".to_string(),
            100000.0,
            "Alice".to_string(),
        );

        assert!(!startup.is_funded());

        let backer = Backer {
            name: "Bob".to_string(),
            amount: 100000.0,
        };

        startup.add_backer(backer);
        assert!(startup.is_funded());
    }
}