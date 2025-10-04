use enrapture::hello;

fn main() {
    println!("{}", hello());

    // the sqlite schema is at src/models/schema.sql
    // use rusqlite to instantiate the database in memory
    let conn = rusqlite::Connection::open("enrapture.db").expect("Failed to open database");
    conn.execute_batch(include_str!("models/schema.sql"))
        .expect("Failed to create schema");

    println!("Database initialized in memory.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
