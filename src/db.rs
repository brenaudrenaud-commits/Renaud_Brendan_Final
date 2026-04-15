use rusqlite::{Connection, Error, params};

pub struct SqlLiteConnection<'a> {
    pub conn: &'a Connection,
}

impl<'a> SqlLiteConnection<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create_fish(
        &self,
        name: &str,
        species: &str,
        length: f64,
        weight: f64,
    ) -> Result<i64, Error> {
        self.conn.execute(
            "INSERT INTO fish (name, species, length, weight) VALUES (?1, ?2, ?3, ?4)",
            params![name, species, length, weight],
        )?;

        Ok(self.conn.last_insert_rowid())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE fish (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                species TEXT NOT NULL,
                length REAL NOT NULL,
                weight REAL NOT NULL
            )",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_create_fish_inserts_row() {
        let conn = setup_test_db();
        let db = SqlLiteConnection::new(&conn);

        let fish_id = db.create_fish("Bubbles", "Trout", 18.5, 4.2).unwrap();

        assert_eq!(fish_id, 1);

        let mut stmt = conn
            .prepare("SELECT id, name, species, length, weight FROM fish WHERE id = ?1")
            .unwrap();

        let fish = stmt
            .query_row([fish_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, f64>(3)?,
                    row.get::<_, f64>(4)?,
                ))
            })
            .unwrap();

        assert_eq!(fish.0, 1);
        assert_eq!(fish.1, "Bubbles");
        assert_eq!(fish.2, "Trout");
        assert_eq!(fish.3, 18.5);
        assert_eq!(fish.4, 4.2);
    }
}