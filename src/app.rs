use std::collections::HashMap;

use crate::{database::Database, game::Game};

pub struct App {
    pub database: Database,
    pub lobbies: HashMap<String, Game>, // maps lobby ID to game
}

impl App {
    pub fn new() -> App {
        App {
            database: Database::new("leet.db"),
            lobbies: HashMap::new(),
        }
    }
}
