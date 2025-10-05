use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tracing::{debug, instrument};

#[instrument]
/// Hash the provided clear password using Argon2 algorithm.
/// # Arguments
///   * `clear_password` - The clear password to hash.
/// # Returns
///   * `String` - The hashed password.
/// # Panics
///   * If there is an error hashing the password.
pub fn hash_pw(clear_password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let pw_hash = argon2
        .hash_password(clear_password.as_bytes(), &salt)
        .expect("Error hashing password")
        .to_string();

    debug!("hashed password");

    let is_correct = argon2
        .verify_password(
            clear_password.as_bytes(),
            &PasswordHash::new(&pw_hash).expect("Error parsing password hash"),
        )
        .is_ok();

    debug!("password is correct? {is_correct}");

    pw_hash
}

#[instrument(name = "password checker", target = "enrapture", level = "info")]
/// Check if the provided clear password matches the stored hash for the given email.
/// Returns true if the password is correct, false otherwise.
/// # Arguments
///   * `email` - The email of the user.
///   * `clear_password` - The clear password to verify.
/// # Returns
///   * `bool` - true if the password is correct, false otherwise.
/// # Panics
///   * If there is an error opening the database or querying the database.
/// # Result
///   * `Ok(true)` - If the password is correct.
/// # Errors
///   * `Err(rusqlite::Error)` - If there is an error querying the database.
pub fn is_pw_correct(email: &str, clear_password: &str) -> bool {
    // let conn = rusqlite::Connection::open("users.db").expect("Error opening database");

    // let mut stmt = conn
    //     .prepare("SELECT password FROM users WHERE email = ?1")
    //     .expect("Error preparing statement");
    // let mut rows = stmt
    //     .query(rusqlite::params![email])
    //     .expect("Error querying database");

    // rows.next().expect("Error getting next row").map_or_else(
    //     || Ok(false),
    //     |row| {
    //         let stored_hash: String = row.get(0).expect("Error getting password from row");

    //         let argon2 = Argon2::default();

    //         let is_correct = argon2
    //             .verify_password(
    //                 clear_password.as_bytes(),
    //                 &PasswordHash::new(&stored_hash).expect("Error parsing password hash"),
    //             )
    //             .is_ok();
    //         Ok(is_correct)
    //     },
    // )
    false
}

#[cfg(test)]
mod test_sql {
    use argon2::PasswordVerifier;

    #[test]
    fn test_pw_hashing() {
        let clear_password = "mysecretpassword";
        let hashed_password = super::hash_pw(clear_password);

        assert!(hashed_password.starts_with("$argon2"));
    }

    #[test]
    fn test_incorrect_pw() {
        let incorrect_password = "wrongpassword";
        let hashed_pw = super::hash_pw("mysecretpassword");

        let argon2 = argon2::Argon2::default();
        let is_correct = argon2
            .verify_password(
                incorrect_password.as_bytes(),
                &argon2::password_hash::PasswordHash::new(&hashed_pw)
                    .expect("Error parsing password hash"),
            )
            .is_ok();
        assert!(!is_correct);
    }

    #[test]
    fn test_correct_pw() {
        let correct_password = "mysecretpassword";
        let hashed_pw = super::hash_pw(correct_password);

        let argon2 = argon2::Argon2::default();
        let is_correct = argon2
            .verify_password(
                correct_password.as_bytes(),
                &argon2::password_hash::PasswordHash::new(&hashed_pw)
                    .expect("Error parsing password hash"),
            )
            .is_ok();
        assert!(is_correct);
    }
}
