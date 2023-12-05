fn main() {
    println!("Encoding 'Hello, world!': {}", encode("Hello, world!"));
}

pub fn encode(input: &str) -> &str {
    return input;
}

pub fn decode(input: &str) -> &str {
    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = "this text is totally rad, dude";
    const ENCODED: &str = "111 01011 1000 1011 110 111 0111 0001 111 110 1000 1011 110 111 10100 111 0110 1001 1001 0100 110 10101 0110 001 01010 110 001 0000 001 0111";

    #[test]
    fn test_encode() {
        assert_eq!(encode(RAW), ENCODED);
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(ENCODED), RAW);
    }

    #[test]
    fn test_both() {
        assert_eq!(decode(encode(RAW)), RAW);
    }
}
