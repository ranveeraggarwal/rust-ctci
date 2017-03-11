#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
fn is_unique_chars_sort(input_string: &str) -> bool {
	let mut char_vec: Vec<char> = input_string.chars().collect();
    char_vec.sort();
	let sorted_char_vec: Vec<char> = char_vec.clone();
    char_vec.dedup();
	return sorted_char_vec == char_vec;
}

#[cfg(test)]
fn is_unique_chars_hash(input_string: &str) -> bool {
    let mut hashmap = HashMap::new();
    for character in input_string.chars() {
        if hashmap.contains_key(&character) {
            return false;
        } else {
            hashmap.insert(character, 1);
        }
    }
    return true;
}

#[cfg(test)]
mod q1_tests {
	use q1::is_unique_chars_sort;

    #[test]
    #[should_panic]
    fn lol_is_not_unique() {
    	assert!(is_unique_chars_sort("lol"));
    }

    #[test]
    fn world_is_unique() {
    	assert!(is_unique_chars_sort("World"));
    }

	#[test]
    fn empty_is_unique() {
    	assert!(is_unique_chars_sort(""));
    }

    #[test]
    fn a_is_unique() {
    	assert!(is_unique_chars_sort("a"));
    }

    use q1::is_unique_chars_hash;
    
    #[test]
    #[should_panic]
    fn lol_is_not_unique_hash() {
        assert!(is_unique_chars_hash("lol"));
    }

    #[test]
    fn world_is_unique_hash() {
        assert!(is_unique_chars_hash("World"));
    }

    #[test]
    fn empty_is_unique_hash() {
        assert!(is_unique_chars_hash(""));
    }

    #[test]
    fn a_is_unique_hash() {
        assert!(is_unique_chars_hash("a"));
    }
}