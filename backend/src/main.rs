#[macro_use]
extern crate rocket;

mod routes;
mod db;

use rocket::{Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/api", routes![routes::users::get_users]) // Ejemplo: Montar ruta básica
}
