use std::sync::Mutex;

use rocket::{State, http::Status, response::status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Deserialize, Serialize)]
pub enum GameType {
    QuickPlay,
    Direct,
    Practice,
}

#[derive(Deserialize, Serialize)]
pub enum GameCategory {
    Calc1,
    Calc2,
    DiscreeteStructures,
    Coding,
    Trivia,
}

#[derive(Deserialize, Serialize)]
pub enum Player {
    Human(String),
    Robot(String),
}

#[derive(Deserialize, Serialize)]
pub struct Game {
    pub game_type: GameType,
    pub game_category: GameCategory,
    pub player1: Player,
    pub player2: Option<Player>,
}

impl Game {}

#[derive(Deserialize, Serialize)]
pub struct InitMatchRequest {
    game_type: GameType,
    authentication: String,
}

#[derive(Deserialize, Serialize)]
pub struct InitMatchResponse {
    game_type: GameType,
    authentication: String,
}

#[post("/init_match", format = "json", data = "<init_request>")]
pub fn init_match(
    state: &State<Mutex<App>>,
    init_request: Json<InitMatchRequest>,
) -> status::Custom<Json<Option<InitMatchResponse>>> {
    let user = state
        .lock()
        .unwrap()
        .database
        .get_user(init_request.authentication.clone());

    let user = if let Some(user) = user {
        user
    } else {
        return status::Custom(Status::BadRequest, Json(None));
    };
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![]
}
