#[macro_use]
extern crate rocket;

mod app;
mod auth;
mod database;
mod game;

use app::App;
use rocket::{Build, Rocket};
use std::sync::Mutex;

#[launch]
async fn rocket() -> _ {
    let app = App::new();
    app.database.init_tables();
    let app = Mutex::new(app);

    rocket::build()
        .mount("/auth", auth::routes())
        .manage(app)
        .configure(rocket::Config {
            ..rocket::Config::default()
        })
}
