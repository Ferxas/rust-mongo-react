use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn connect() -> Client {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let options = ClientOptions::parse(mongo_uri)
        .await
        .expect("Failed to parse MongoDB options");
    Client::with_options(options).expect("Failed to connect to MongoDB")
}
