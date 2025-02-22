use rocket::tokio::sync::RwLock;
use std::sync::Arc;

use crate::database::Database;

pub struct App {
    pub database: Database,
}

impl App {
    pub fn new() -> App {
        App {
            database: Database::new("leet.db"),
        }
    }
}

pub type SharedApp = Arc<RwLock<App>>;
