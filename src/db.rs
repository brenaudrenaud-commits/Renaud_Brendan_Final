use rusqlite::{Connection, Error, params};

pub struct SqlLiteConnection<'a> {
    pub conn: &'a Connection,
}

//implement the connection to SQlite database
impl<'a> SqlLiteConnection<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        return Self {
            conn,
        };
    }

    pub fn create_fish(&mut self, name: &String, email: &String) -> Result<i64, Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO FISH (name, species, lenght, weight) VALUES ()"
        )?;
        stmt.execute(params![(":name", name), (":email", email)])?;
        Ok(self.conn.last_insert_rowid())
    }
}
