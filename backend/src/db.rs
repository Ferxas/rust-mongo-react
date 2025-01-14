use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn get_database() -> Database {
    // URI database since mongodb
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI debe estar configurado en el archivo .env");
    
    // config options of client
    let options = ClientOptions::parse(mongo_uri)
        .await
        .expect("No se pudieron parsear las opciones de MongoDB");
    
    let client = Client::with_options(options).expect("No se pudo conectar a MongoDB");

    client.database("rustFullStack") // db_name
}
