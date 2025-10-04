use enrapture::{
    hello,
    models::sqlite::{DATABASE_NAME, create_user},
};

fn main() {
    println!("{}", hello());

    // the sqlite schema is at src/models/schema.sql
    // use rusqlite to instantiate the database in memory
    let conn = rusqlite::Connection::open(DATABASE_NAME).expect("Failed to open database");
    conn.execute_batch(include_str!("models/schema.sql"))
        .expect("Failed to create schema");
    // close the connection
    drop(conn);

    println!("Database initialized in memory.");

    for _ in 0..5 {
        let random_email_number = rand::random::<u8>();

        let email = format!("user_{random_email_number}");

        // Create an arbitrary user
        create_user(&format!("{email}@email.com"), "password123");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
