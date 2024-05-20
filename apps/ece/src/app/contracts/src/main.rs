// src/main.rs
mod db;
mod entity;

use db::connect;
use entity::prelude::*;
use sea_orm::*;
use dotenv::dotenv;
use sea_orm::entity::Set;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db: DatabaseConnection = connect().await;

    // Create
    let user = user::ActiveModel {
        name: Set("Alice".to_owned()),
        ..Default::default()
    };
    let user: user::Model = user.insert(&db).await.unwrap();
    println!("Created user: {:?}", user);

    // Read
    let user: Option<user::Model> = User::find_by_id(user.id).one(&db).await.unwrap();
    println!("Fetched user: {:?}", user);

    // Update
    let mut user: user::ActiveModel = user.unwrap().into();
    user.name = Set("Alice Updated".to_owned());
    let user: user::Model = user.update(&db).await.unwrap();
    println!("Updated user: {:?}", user);

    // Delete
    let res = User::delete_by_id(user.id).exec(&db).await.unwrap();
    println!("Deleted user: {:?}", res);
}
