use rusqlite::{Connection, Error, params}; //import crate with needed imports struct, enum, macro (params)
use crate::fish::Fish;

pub struct SqLiteConnection<'a> {
    pub conn: &'a Connection,
}

impl<'a> SqLiteConnection<'a> {
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

        Ok(self.conn.last_insert_rowid())//returning result enum
    }


//display fish
pub fn get_all_fish(&self) -> Result<Vec<Fish>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, species, length, weight FROM fish ORDER BY id ASC"
        )?;

        let fish_iter = stmt.query_map([], |row| {
            Ok(Fish {
                id: row.get(0)?,
                name: row.get(1)?,
                species: row.get(2)?,
                length: row.get(3)?,
                weight: row.get(4)?,
            })
        })?;

        let mut fish_list = Vec::new();

        for fish in fish_iter {
            fish_list.push(fish?);
        }

        Ok(fish_list)//returning result enum
    }
}

#[cfg(test)]
mod tests {
    //import everything from the parent modules
    use super::*;
    //setup a temporary database connection for the tests
    use rusqlite::Connection;

    //fn to test on our temporary connection
    fn setup_test_db() -> Connection {
        //open temp connection
        let conn = Connection::open_in_memory().unwrap();

        //create inatial fish table just like i did in main.rs on line 41 but temporary here
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

        // return the database connection
        conn
    }

    #[test]
    fn test_create_fish_inserts_row() {
        //new temporary database
        let conn = setup_test_db();

        //borrow database connection calling the new function
        let db = SqLiteConnection::new(&conn);

        //test insert one fish row into the fish table and return id
        let fish_id = db.create_fish("Bubbles", "Trout", 18.5, 4.2).unwrap();

        //since this is a temporary connection/test the id should be 1
        //indicating we had no fish and inserted 1 new fish
        assert_eq!(fish_id, 1);

        //sql query to read the row that was just inserted
        let mut stmt = conn
            .prepare("SELECT id, name, species, length, weight FROM fish WHERE id = ?1")
            .unwrap();

        //run the query prepared on the returned fish id(current fish)
        //query expects one match per row 
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

        //verify the query has expected values
        assert_eq!(fish.0, 1);
        assert_eq!(fish.1, "Bubbles");
        assert_eq!(fish.2, "Trout");
        assert_eq!(fish.3, 18.5);
        assert_eq!(fish.4, 4.2);
    }

#[test]
fn test_get_all_fish_returns_inserted_fish() {
    let conn = setup_test_db();
    let db = SqLiteConnection::new(&conn);

    db.create_fish("Bubbles", "Trout", 18.5, 4.2).unwrap();
    db.create_fish("Splash", "Bass", 12.0, 2.1).unwrap();

    let fish_list = db.get_all_fish().unwrap();

    assert_eq!(fish_list.len(), 2);
    assert_eq!(fish_list[0].name, "Bubbles");
    assert_eq!(fish_list[0].species, "Trout");
    assert_eq!(fish_list[1].name, "Splash");
    assert_eq!(fish_list[1].species, "Bass");
}
}