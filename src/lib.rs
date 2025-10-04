pub mod models;
pub mod security;

#[must_use]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[must_use]
pub fn hello() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
