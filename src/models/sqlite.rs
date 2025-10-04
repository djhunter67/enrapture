use crate::security::hashing::hash_pw;

pub const DATABASE_NAME: &str = "enrapture.db";

/// Create a new user with the given email and clear password.
/// The password is hashed using argon2 before being stored in the database.
/// # Arguments
///   * `email` - The email of the user.
///   * `clear_password` - The clear password of the user.
/// # Returns
///   * `()` - Nothing.
/// # Panics
/// * If there is an error opening the database or inserting the user.
pub fn create_user(email: &str, clear_password: &str) {
    // hash the password using argon2
    let hashed_password = hash_pw(clear_password);
    let conn = rusqlite::Connection::open(DATABASE_NAME).expect("Error opening database");

    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES (?1, ?2)",
        rusqlite::params![email, hashed_password],
    )
    .expect("Error inserting user");
    drop(conn);

    println!("\n{email} created successfully\n");
}
