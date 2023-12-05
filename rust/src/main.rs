fn main() {
    println!(
        "Encoding 'Hello, world!': {}",
        encode(String::from("Hello, world!"))
    );
}

pub fn encode(input: String) -> String {
    return input;
}

pub fn decode(input: String) -> String {
    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(String::from("this text is totally rad, dude")), String::from("111 01011 1000 1011 110 111 0111 0001 111 110 1000 1011 110 111 10100 111 0110 1001 1001 0100 110 10101 0110 001 01010 110 001 0000 001 0111"));
    }
}
