use argon2::{self, Config, verify_encoded};
use rand::{self, Rng};
use rusqlite::{Connection, Result, params};
use std::{
    time::{SystemTime, UNIX_EPOCH},
    u64,
};

use crate::auth::Token;

pub struct Database {
    connection: Connection,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub token: String,
    pub elo: u32,
    pub time_created: String,
    pub games_won: u32,
    pub games_lost: u32,
}

impl Database {
    pub fn new(path: &str) -> Database {
        let connection = Connection::open(path).unwrap();

        Database { connection }
    }

    fn hash_password(password: String) -> String {
        let salt: [u8; 16] = [
            69, 42, 69, 42, 69, 42, 69, 42, 69, 42, 69, 42, 69, 42, 69, 42,
        ];

        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
    }

    pub fn init_tables(&self) {
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                    username TEXT PRIMARY KEY,
                    hashed_password TEXT NOT NULL,
                    token TEXT NOT NULL,
                    elo INTEGER NOT NULL,
                    time_created TEXT NOT NULL,
                    games_won INTEGER NOT NULL,
                    games_lost INTEGER NOT NULL
                )",
                [],
            )
            .unwrap();
    }

    pub fn register_user(&self, username: String, password: String) -> Option<Token> {
        let hashed_password = Database::hash_password(password.clone());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let token =
            Database::hash_password(rand::thread_rng().gen_range(u64::MIN..u64::MAX).to_string());

        let result = self.connection.execute(
            "INSERT into users (
                username,
                hashed_password,
                token,
                elo,
                time_created,
                games_won,
                games_lost
            )
            VALUES (
                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7
            )",
            (&username, &hashed_password, token, 500, timestamp, 0, 0),
        );

        if result.is_ok() {
            self.get_token(username, password)
        } else {
            None
        }
    }

    pub fn get_token(&self, username: String, password: String) -> Option<Token> {
        let mut stmt = self
            .connection
            .prepare("SELECT token, hashed_password FROM users WHERE username = ?1")
            .unwrap();

        let mut rows = stmt.query([username]).unwrap();
        let mut results = Vec::<(String, String)>::new();

        while let Some(row) = rows.next().unwrap() {
            let token: String = row.get(0).unwrap();
            let password: String = row.get(1).unwrap();
            results.push((token, password));
        }

        let hashed_password = Database::hash_password(password);

        for user in results {
            if user.1 == hashed_password {
                return Some(user.0);
            }
        }

        None
    }
}
