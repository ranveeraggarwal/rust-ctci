#[cfg(test)]
fn compress(input_string: &str) -> String {
    let mut counter: u8 = 0;
    let mut current: char = ' ';
    let mut init: bool = true;
    let mut compressed_string: String = String::new();
    for character in input_string.chars() {
        if init {
            current = character;
            counter += 1;
            init = false;
        } else {
            if character == current {
                counter += 1;
            } else {
                compressed_string.push(current);
                compressed_string.push_str(&(counter.to_string()));
                current = character;
                counter = 1;
            }
        }
    }
    compressed_string.push(current);
    compressed_string.push_str(&(counter.to_string()));

    if compressed_string.len() >= input_string.len() {
        return input_string.to_string();
    } else {
        return compressed_string;
    }
}

#[cfg(test)]
mod q6_tests {
    use ch1::q6::compress;

    #[test]
    fn compress_aabcccccaaa() {
        assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
    }

    #[test]
    fn compress_abcdef() {
        assert_eq!(compress("abcdef"), "abcdef");
    }

    #[test]
    fn compress_nothing() {
        assert_eq!(compress(""), "");
    }
}