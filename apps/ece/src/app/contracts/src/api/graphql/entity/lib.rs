// src/lib.rs
use sea_orm::*;
use tokio::main;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Contract {
    db: DatabaseConnection,
}

impl Contract {
    pub async fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = Database::connect(&database_url).await.expect("Failed to connect to database");
        Self { db }
    }

    pub async fn get_quiz_questions(&self) -> Vec<quizzes::Model> {
        Quizzes::find().all(&self.db).await.expect("Error loading quizzes")
    }

    pub async fn submit_quiz_answers(&self, user_id: i32, answers: Vec<quiz_answers::Model>) -> bool {
        let mut correct_count = 0;
        for answer in answers.iter() {
            let quiz = Quizzes::find_by_id(answer.quiz_id)
                .one(&self.db)
                .await
                .expect("Error finding quiz")
                .expect("Quiz not found");

            let mut updated_answer = answer.clone();
            updated_answer.is_correct = quiz.correct_answer == answer.given_answer;
            if updated_answer.is_correct {
                correct_count += 1;
            }

            quiz_answers::ActiveModel {
                answer_id: Set(updated_answer.answer_id),
                user_id: Set(updated_answer.user_id),
                quiz_id: Set(updated_answer.quiz_id),
                given_answer: Set(updated_answer.given_answer.clone()),
                is_correct: Set(updated_answer.is_correct),
            }
            .update(&self.db)
            .await
            .expect("Error inserting answer");
        }

        correct_count == answers.len()
    }

    // Add NEAR SDK interaction for rewarding here
}

// src/lib.rs (continue)
use near_sdk::{env, near_bindgen, AccountId, Promise};

// Add NEAR SDK interaction to reward user
#[near_bindgen]
impl Contract {
    pub fn reward_user(&self, user_id: AccountId, reward_amount: u128) {
        assert_eq!(env::predecessor_account_id(), self.admin_account_id(), "Only admin can reward");
        Promise::new(user_id).transfer(reward_amount);
    }

    // Helper function to get admin account ID
    pub fn admin_account_id(&self) -> AccountId {
        env::current_account_id()
    }
}

// Main function to set up contract and run the application
#[tokio::main]
async fn main() {
    let contract = Contract::new().await;

    // Example: Fetching quiz questions
    let quizzes = contract.get_quiz_questions().await;
    println!("{:?}", quizzes);

    // Example: Submitting quiz answers and rewarding user if all answers are correct
    let user_id = 1;
    let answers = vec![
        quiz_answers::Model {
            answer_id: 0,
            user_id,
            quiz_id: 1,
            given_answer: "Correct Answer".to_string(),
            is_correct: false,
        },
        // more answers...
    ];

    let all_correct = contract.submit_quiz_answers(user_id, answers).await;
    if all_correct {
        contract.reward_user("user.near".to_string(), 1000000000000000000000000);
    }
}