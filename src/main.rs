use enrapture::hello;

fn main() {
    println!("{}", hello());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
