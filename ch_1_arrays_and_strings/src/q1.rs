#[cfg(test)]
fn is_unique_chars(input_string: &str) -> bool {
	let mut char_vec: Vec<char> = input_string.chars().collect();
    char_vec.sort();
	let sorted_char_vec: Vec<char> = char_vec.clone();
    char_vec.dedup();
	sorted_char_vec == char_vec
}

#[cfg(test)]
mod q1_tests {
	use q1::is_unique_chars;
    #[test]
    #[should_panic]
    fn lol_is_not_unique() {
    	assert!(is_unique_chars("lol"));
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