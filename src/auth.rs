use crate::{app::App, database::User};
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

#[derive(Deserialize)]
pub struct Authorization {
    authorization: String,
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

#[post("/get_user", format = "json", data = "<authorization>")]
fn get_user(
    state: &State<Mutex<App>>,
    authorization: Json<Authorization>,
) -> status::Custom<Json<Option<User>>> {
    let response = state
        .lock()
        .unwrap()
        .database
        .get_user(authorization.authorization.clone());

    match response {
        Some(user) => {
            return status::Custom(Status::Ok, Json(Some(user)));
        }
        None => {
            return status::Custom(Status::BadRequest, Json(None));
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![login, register, get_user]
}
