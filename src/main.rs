use enrapture::{
    hello,
    models::sqlite::{DATABASE_NAME, create_user},
};
use tracing::{Level, info, level_filters::LevelFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt};

fn main() {
    let rolling_log = RollingFileAppender::new(Rotation::NEVER, "./", "enrapped");

    let (non_blocking, _) = tracing_appender::non_blocking(rolling_log);

    let layer_1 = fmt::Layer::default()
        .with_writer(non_blocking)
        .with_filter(LevelFilter::from(Level::DEBUG));

    let (non_blocking, _) = tracing_appender::non_blocking(std::io::stdout());

    let layer_2 = fmt::Layer::default()
        .with_writer(non_blocking)
        .with_filter(LevelFilter::from(Level::TRACE));

    let subscriber = tracing_subscriber::registry().with(layer_1).with(layer_2);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    tracing::info!("Hello, world; from tracing");

    println!("{}", hello());

    // the sqlite schema is at src/models/schema.sql
    // use rusqlite to instantiate the database in memory
    let conn = rusqlite::Connection::open(DATABASE_NAME).expect("Failed to open database");
    conn.execute_batch(include_str!("models/schema.sql"))
        .expect("Failed to create schema");
    // close the connection
    drop(conn);

    info!("Database initialized in memory.");

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
