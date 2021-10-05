use serenity::{
    prelude::*,
};
use anyhow::Result;
use std::path::Path;
use rusqlite::{Connection, params};
use typemap_rev::TypeMapKey;

pub struct Database {
    conn: Mutex<Connection>
}

impl TypeMapKey for Database {
    type Value = Database;
}

// Connect to the sqlite3 database
pub fn connect() -> Result<Database> {
    let conn = Connection::open(Path::new("runners.db"))?;
    conn.execute("CREATE TABLE IF NOT EXISTS runners (runner TEXT, last_run TEXT)", [])?;
    let db = Database {conn: Mutex::new(conn)};
    Ok(db)
}

impl Database {
    // Add a new runner
    pub async fn add_runner(&self, runner: &str, last_run: &str) -> Result<()> {
        let conn = &self.conn.lock().await;
        conn.execute("INSERT INTO runners VALUES (?1, ?2)", params![runner, last_run])?;
        Ok(())
    }

    // Update runner's last run
    pub async fn update_runner(&self, runner: String, last_run: String) -> Result<()> {
        let conn = &self.conn.lock().await;
        conn.execute("UPDATE runners SET last_run = ?1 WHERE runner = ?2", params![last_run, runner])?;
        Ok(())
    }

    // Get all runners
    pub async fn get_runners(&self) -> Result<Vec<Runner>> {
        let conn = &self.conn.lock().await;
        let mut statement = conn.prepare("SELECT * FROM runners")?;
        let runners = statement.query_map([], |row| {
            Ok(Runner {
                name: row.get(0)?,
                last_run: row.get(1)?,
            })
        })?;
        let mut runners_vector: Vec<Runner> = Vec::new();
        for runner in runners {
            runners_vector.push(runner.unwrap());
        }
        Ok(runners_vector)
    }
}

#[derive(Debug)]
pub struct Runner {
    pub name: String,
    pub last_run: String
}