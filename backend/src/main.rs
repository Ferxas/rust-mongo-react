use dotenvy::dotenv;
use std::env;

#[macro_use]
extern crate rocket;

mod routes;
mod db;
mod models;

use rocket::{Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    // loading .env
    dotenv().ok();
    rocket::build()
        .mount("/api", routes![routes::users::create_user, routes::users::list_users])
}