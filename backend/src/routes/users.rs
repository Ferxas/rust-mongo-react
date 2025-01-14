use rocket::serde::json::{Json, Value};
use serde_json::json;
use crate::models::user::User;

#[post("/users", format = "json", data = "<user>")]
pub async fn create_user(user: Json<User>) -> Value {
    let user_data = user.into_inner();

    match User::create_user(user_data).await {
        Ok(_) => json!({ "status": "success", "message": "User created successfully" }),
        Err(e) => json!({ "status": "error", "message": e.to_string() }),
    }
}

#[get("/users")]
pub async fn list_users() -> Value {
    // Esta ruta devolverá la lista de usuarios (implementación futura)
    json!({ "status": "success", "message": "Fetch users is under construction" })
}
