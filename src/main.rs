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
        .mount("/game", game::routes())
        .manage(app)
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            ..rocket::Config::default()
        })
}
