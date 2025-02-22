use crate::app::App;
use rocket::{State, http::Status, response::status, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize)]
pub struct AuthorizationResponse {
    pub authorization: Option<String>,
}

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub type Token = String;

#[post("/login", format = "json", data = "<credentials>")]
pub fn login(
    state: &State<Mutex<App>>,
    credentials: Json<Credentials>,
) -> status::Custom<Json<AuthorizationResponse>> {
    let response = state
        .lock()
        .unwrap()
        .database
        .get_token(credentials.username.clone(), credentials.password.clone());

    match response {
        Some(token) => {
            return status::Custom(
                Status::Ok,
                Json(AuthorizationResponse {
                    authorization: Some(token),
                }),
            );
        }
        None => {
            return status::Custom(
                Status::BadRequest,
                Json(AuthorizationResponse {
                    authorization: None,
                }),
            );
        }
    }
}

#[post("/register", format = "json", data = "<credentials>")]
pub fn register(
    state: &State<Mutex<App>>,
    credentials: Json<Credentials>,
) -> status::Custom<Json<AuthorizationResponse>> {
    let response = state
        .lock()
        .unwrap()
        .database
        .register_user(credentials.username.clone(), credentials.password.clone());

    match response {
        Some(token) => {
            println!("hello");
            return status::Custom(
                Status::Ok,
                Json(AuthorizationResponse {
                    authorization: Some(token),
                }),
            );
        }
        None => {
            return status::Custom(
                Status::BadRequest,
                Json(AuthorizationResponse {
                    authorization: None,
                }),
            );
        }
    }
}

#[post("/test2", format = "json", data = "<credentials>")]
fn test2(state: &State<Mutex<App>>, credentials: Json<Credentials>) -> String {
    let username = &credentials.username;
    let password = &credentials.password;

    // Here you can process the credentials, e.g., authentication
    format!("Username: {}, Password: {}", username, password)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![login, register, test2]
}
