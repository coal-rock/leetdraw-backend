use std::sync::{Arc, Mutex, RwLock};

use rocket::{State, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::app::{App, SharedApp};

#[derive(Serialize, Deserialize, FromForm)]
pub struct Login {
    pub username: String,
    pub passsword: String,
}

pub type Token = String;

#[post("/login", format = "json", data = "<login>")]
pub fn login(state: &State<Mutex<App>>, login: Json<Login>) -> Option<Token> {
    state
        .lock()
        .unwrap()
        .database
        .get_token(login.username.clone(), login.passsword.clone())
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![login]
}
