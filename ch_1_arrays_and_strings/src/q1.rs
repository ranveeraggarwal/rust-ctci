#[cfg(test)]
fn is_unique_chars(input_string: &str) -> bool {
	let mut char_vec: Vec<char> = input_string.chars().collect();
	char_vec.dedup();
	let unique_string: String = char_vec.into_iter().collect();
	unique_string == input_string
}

#[cfg(test)]
mod q1_tests {
	use q1::is_unique_chars;
    #[test]
    #[should_panic]
    fn hello_is_not_unique() {
    	assert!(is_unique_chars("Hello"));
    }

    #[test]
    fn world_is_unique() {
    	assert!(is_unique_chars("World"));
    }

	#[test]
    fn empty_is_unique() {
    	assert!(is_unique_chars(""));
    }

    #[test]
    fn a_is_unique() {
    	assert!(is_unique_chars("a"));
    }
}