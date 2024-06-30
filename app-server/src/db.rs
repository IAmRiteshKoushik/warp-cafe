use rusqlite::Connection;

pub fn get_db_conn() -> Connection {
    let conn = Connection::open("restuarant.db").expect("Failed to open SQLite connection");
}
