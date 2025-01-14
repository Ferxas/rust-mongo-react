use serde::{Deserialize, Serialize};
use mongodb::bson::{self, oid::ObjectId};
use mongodb::{Collection, error::Result};
use crate::db::get_database;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDB identifier
    pub name: String,
    pub email: String,
}


impl User {
    pub async fn create_user(user: User) -> mongodb::error::Result<()> {
        let db = get_database().await; // database connection
        let collection: Collection<User> = db.collection("users");

        // insert document
        collection.insert_one(user).await?; // Solo pasa `user` como argumento
        
        // reference: https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.insert_one
        Ok(())
    }
}
