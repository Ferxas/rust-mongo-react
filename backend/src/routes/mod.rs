pub mod users;

use rocket::serde::json::Json;

#[get("/users")]
pub fn get_users() -> Json<&'static str> {
    Json("{\"Message:\"}: \"Hello, Users!\"}")
}