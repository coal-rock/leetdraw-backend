use std::collections::HashMap;

use crate::{
    database::Database,
    game::{Game, GameType},
};

pub struct App {
    pub database: Database,
    pub games: HashMap<String, Game>,
}

impl App {
    pub fn new() -> App {
        App {
            database: Database::new("leet.db"),
            games: HashMap::new(),
        }
    }

    pub fn list_open_quickplay_games(&self) -> Option<HashMap<String, Game>> {
        let mut open_games: HashMap<String, &Game> = HashMap::new();

        for (username, game) in self.games.iter() {
            match game.game_type {
                GameType::QuickPlay => match game.player2 {
                    Some(_) => {
                        open_games.insert(username.to_string(), self.games.get(username).unwrap());
                    }
                    None => {}
                },
                _ => {}
            }
        }

        if open_games.len() > 0 {
            Some(open_games)
        } else {
            None
        }
    }
}
