use crate::security::hashing::hash_pw;

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
    let conn = rusqlite::Connection::open("users.db").expect("Error opening database");

    conn.execute(
        "INSERT INTO users (email, password) VALUES (?1, ?2, ?3)",
        rusqlite::params![email, hashed_password],
    )
    .expect("Error inserting user");

    println!("{email} created successfully");
}
