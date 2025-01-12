use rusqlite::{params, Connection, OptionalExtension};

use crate::Storage;

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn new(db_path: &str) -> Result<Self, String> {
        let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (name TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )
        .map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }
}

impl Storage for SqliteStorage {
    fn write(&self, key: &str, value: &str) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT INTO secrets (name, value) VALUES (?1, ?2) ON CONFLICT(name) DO UPDATE SET value = excluded.value",
                params![key, value],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn read(&self, key: &str) -> Result<Option<String>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM secrets WHERE name = ?1")
            .map_err(|e| e.to_string())?;
        let result: Result<Option<String>, _> = stmt
            .query_row(params![key], |row| row.get(0))
            .optional();
        result.map_err(|e| e.to_string())
    }

    fn update(&self, key: &str, value: &str) -> Result<(), String> {
        let rows_updated = self
            .conn
            .execute(
                "UPDATE secrets SET value = ?1 WHERE name = ?2",
                params![value, key],
            )
            .map_err(|e| e.to_string())?;
        
        if rows_updated == 0 {
            Err(format!("Key '{}' does not exist", key))
        } else {
            Ok(())
        }
    }    

    fn get_all(&self) -> Result<Vec<(String, String)>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, value FROM secrets")
            .map_err(|e| e.to_string())?;
        let results = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                Ok((key, value))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<(String, String)>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(results)
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM secrets WHERE name = ?1", params![key])
            .map_err(|e| e.to_string())
            .map(|_| ())
    }
}

