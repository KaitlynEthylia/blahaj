use std::{env, fmt};
use std::fmt::Debug;
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use color_eyre::eyre::Result;
use diesel::prelude::Connection as _;
use diesel::sqlite::SqliteConnection;

pub struct Connection {
    pub db: Arc<Mutex<SqliteConnection>>,
}

impl Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CONNECTION")
    }
}

impl Connection {
    pub fn connect() -> Result<Self> {
        let path = PathBuf::from(env::var("DATABASE_PATH")?);

        if !path.exists() {
            let _ = File::create(&path);
        }

        let url = format!("file:{}", path.display());
        let connection = SqliteConnection::establish(&url)?;

        Ok(Connection {
            db: Arc::new(Mutex::new(connection)),
        })
    }
}
