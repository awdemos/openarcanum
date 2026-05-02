use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, OptionalExtension};

use crate::{Character, Error};

/// SQLite-backed character store for persistent character storage.
#[derive(Debug, Clone)]
pub struct SqliteCharacterStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCharacterStore {
    /// Open or create a SQLite database at the given path.
    pub fn open(path: &str) -> Result<Self, Error> {
        let conn = Connection::open(path).map_err(|e| {
            Error::Storage(format!("Failed to open SQLite database: {e}"))
        })?;
        let store = SqliteCharacterStore {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init()?;
        Ok(store)
    }

    /// Create an in-memory SQLite database (useful for testing).
    pub fn open_in_memory() -> Result<Self, Error> {
        let conn = Connection::open_in_memory().map_err(|e| {
            Error::Storage(format!("Failed to open in-memory SQLite database: {e}"))
        })?;
        let store = SqliteCharacterStore {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init()?;
        Ok(store)
    }

    fn init(&self) -> Result<(), Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS characters (
                id TEXT PRIMARY KEY,
                system_id TEXT NOT NULL,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| Error::Storage(format!("Failed to create characters table: {e}")))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_characters_system ON characters(system_id)",
            [],
        )
        .map_err(|e| Error::Storage(format!("Failed to create index: {e}")))?;

        Ok(())
    }
}

impl crate::system::CharacterStore for SqliteCharacterStore {
    fn save(&mut self, character: &Character) -> Result<(), Error> {
        let now = chrono::Utc::now().to_rfc3339();
        let data = serde_json::to_string(character)
            .map_err(|e| Error::Serialization(e.to_string()))?;

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO characters (id, system_id, name, data, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET
             system_id = excluded.system_id,
             name = excluded.name,
             data = excluded.data,
             updated_at = excluded.updated_at",
            params![
                character.id.to_string(),
                character.system_id.to_string(),
                character.identity.name.clone(),
                data,
                now.clone(),
                now,
            ],
        )
        .map_err(|e| Error::Storage(format!("Failed to save character: {e}")))?;

        Ok(())
    }

    fn load(&self, id: &str) -> Result<Option<Character>, Error> {
        let conn = self.conn.lock().unwrap();
        let data: Option<String> = conn
            .query_row(
                "SELECT data FROM characters WHERE id = ?1",
                [id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| Error::Storage(format!("Failed to load character: {e}")))?;

        match data {
            Some(json) => {
                let character: Character = serde_json::from_str(&json)
                    .map_err(|e| Error::Serialization(e.to_string()))?;
                Ok(Some(character))
            }
            None => Ok(None),
        }
    }

    fn delete(&mut self, id: &str) -> Result<(), Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM characters WHERE id = ?1",
            [id],
        )
        .map_err(|e| Error::Storage(format!("Failed to delete character: {e}")))?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Character>, Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT data FROM characters ORDER BY updated_at DESC")
            .map_err(|e| Error::Storage(format!("Failed to list characters: {e}")))?;

        let rows = stmt
            .query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })
            .map_err(|e| Error::Storage(format!("Failed to query characters: {e}")))?;

        let mut characters = Vec::new();
        for row in rows {
            let data = row.map_err(|e| Error::Storage(e.to_string()))?;
            let character: Character = serde_json::from_str(&data)
                .map_err(|e| Error::Serialization(e.to_string()))?;
            characters.push(character);
        }

        Ok(characters)
    }

    fn list_by_system(&self, system_id: &str) -> Result<Vec<Character>, Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT data FROM characters WHERE system_id = ?1 ORDER BY updated_at DESC")
            .map_err(|e| Error::Storage(format!("Failed to list characters by system: {e}")))?;

        let rows = stmt
            .query_map([system_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })
            .map_err(|e| Error::Storage(format!("Failed to query characters by system: {e}")))?;

        let mut characters = Vec::new();
        for row in rows {
            let data = row.map_err(|e| Error::Storage(e.to_string()))?;
            let character: Character = serde_json::from_str(&data)
                .map_err(|e| Error::Serialization(e.to_string()))?;
            characters.push(character);
        }

        Ok(characters)
    }
}
