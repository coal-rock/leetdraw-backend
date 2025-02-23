use std::sync::Mutex;

use rand::Rng;
use rocket::{State, http::Status, response::status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::{app::App, auth::Authorization};

#[derive(Deserialize, Serialize)]
pub enum GameCategory {
    Calc1,
    Calc2,
    DiscreeteStructures,
    Coding,
    Trivia,
}

#[derive(Deserialize, Serialize)]
pub struct Game {
    pub game_category: GameCategory,
    pub player1: String,
    pub player2: Option<String>,
}

impl Game {}

#[derive(Deserialize, Serialize)]
pub struct InitMatchRequest {
    // game_category: GameCategory,
    authorization: String,
}

#[derive(Deserialize, Serialize)]
pub struct InitMatchResponse {
    success: bool,
    game_id: String,
}

// Adds user to matchmaking queue for specific category
#[post("/init_match", format = "json", data = "<init_request>")]
pub fn init_match(
    state: &State<Mutex<App>>,
    init_request: Json<InitMatchRequest>,
) -> status::Custom<Json<Option<InitMatchResponse>>> {
    let init_request = init_request.0;

    let user = state
        .lock()
        .unwrap()
        .database
        .get_user(init_request.authorization.clone());

    let user = match user {
        Some(user) => user,
        None => return status::Custom(Status::BadRequest, Json(None)),
    };

    if state.lock().unwrap().lobbies.is_empty() {
        let game_id = rand::thread_rng().gen_range(u64::MIN..u64::MAX).to_string();

        let game = Game {
            game_category: GameCategory::Calc2,
            player1: user.username,
            player2: None,
        };

        state.lock().unwrap().lobbies.insert(game_id.clone(), game);

        return status::Custom(
            Status::Ok,
            Json(Some(InitMatchResponse {
                success: true,
                game_id,
            })),
        );
    } else {
        for (id, game) in &mut state.lock().unwrap().lobbies {
            if game.player2.is_none() && game.player1 != user.username.clone() {
                game.player2 = Some(user.username.clone());

                return status::Custom(
                    Status::Ok,
                    Json(Some(InitMatchResponse {
                        success: true,
                        game_id: id.to_string(),
                    })),
                );
            }
        }

        let game_id = rand::thread_rng().gen_range(u64::MIN..u64::MAX).to_string();

        let game = Game {
            game_category: GameCategory::Calc2,
            player1: user.username,
            player2: None,
        };

        state.lock().unwrap().lobbies.insert(game_id.clone(), game);

        return status::Custom(
            Status::Ok,
            Json(Some(InitMatchResponse {
                success: true,
                game_id,
            })),
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct PollMatchRequest {
    authorization: String,
    game_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PollMatchResponse {
    game_ready: bool,
    message: Option<String>,
}

#[post("/poll_match", format = "json", data = "<poll_match_request>")]
pub fn poll_match(
    state: &State<Mutex<App>>,
    poll_match_request: Json<PollMatchRequest>,
) -> status::Custom<Json<Option<PollMatchResponse>>> {
    let poll_match_request = poll_match_request.0;

    let user = state
        .lock()
        .unwrap()
        .database
        .get_user(poll_match_request.authorization.clone());

    let user = match user {
        Some(user) => user,
        None => {
            return status::Custom(
                Status::BadRequest,
                Json(Some(PollMatchResponse {
                    game_ready: false,
                    message: Some(String::from("invalid user authorization")),
                })),
            );
        }
    };

    match state
        .lock()
        .unwrap()
        .lobbies
        .get(&poll_match_request.game_id)
    {
        Some(game) => {
            if game.player2.is_some() {
                return status::Custom(
                    Status::Ok,
                    Json(Some(PollMatchResponse {
                        game_ready: true,
                        message: None,
                    })),
                );
            } else {
                return status::Custom(
                    Status::Ok,
                    Json(Some(PollMatchResponse {
                        game_ready: false,
                        message: Some(String::from("lobby missing second player")),
                    })),
                );
            }
        }
        None => {
            return status::Custom(
                Status::BadRequest,
                Json(Some(PollMatchResponse {
                    game_ready: false,
                    message: Some(String::from("invalid game id")),
                })),
            );
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![init_match, poll_match]
}
