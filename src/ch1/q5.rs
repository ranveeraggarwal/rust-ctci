#[cfg(test)]
fn one_away(string_1: &str, string_2: &str) -> bool {
	let char_vec_1: Vec<char> = string_1.chars().collect();
	let char_vec_2: Vec<char> = string_2.chars().collect();
	let l1: usize = char_vec_1.len();
	let l2: usize = char_vec_2.len();
	if l1 == l2 {
		return same_length(&char_vec_1, &char_vec_2);
	} else if l1 as i32 - l2 as i32 == 1 {
		return diff_length(&char_vec_1, &char_vec_2);
	} else if l2 as i32 - l1 as i32 == 1 {
		return diff_length(&char_vec_2, &char_vec_1);
	} else {
		return false;
	}
}

#[cfg(test)]
fn same_length(char_vec_1: &Vec<char>, char_vec_2: &Vec<char>) -> bool {
	let mut found_once: bool = false;
	for i in 0..char_vec_1.len() {
		if char_vec_1[i] != char_vec_2[i] {
			if found_once == true {
				return false;
			}
			found_once = true;
		}
	}
	return true;
}

#[cfg(test)]
fn diff_length(char_vec_l: &Vec<char>, char_vec_s: &Vec<char>) -> bool {
	let mut index_l: usize = 0;
	let mut index_s: usize = 0;
	while (index_l < char_vec_l.len()) && (index_s < char_vec_s.len()) {
		if char_vec_l[index_l] != char_vec_s[index_s] {
			if index_l != index_s {
				return false;
			}
			index_l += 1;
		} else {
			index_l += 1;
			index_s += 1;
		}
	}
	return true;
}

#[cfg(test)]
mod q5_tests {
	use ch1::q5::one_away;

	#[test]
	fn one_away_ple_pale() {
		assert!(one_away("ple", "pale"));
	}

	#[test]
	fn one_away_pales_pale() {
		assert!(one_away("pales", "pale"));
	}

	#[test]
	fn one_away_pale_bale() {
		assert!(one_away("pale", "bale"));
	}

	#[test]
	fn one_away_not_pale_bake() {
		assert_eq!(one_away("pale", "bake"), false);
	}

	#[test]
	fn one_away_ple_pales() {
		assert_eq!(one_away("ple", "pales"), false);
	}

	#[test]
	fn one_away_ple_pbxe() {
		assert_eq!(one_away("ple", "pbxe"), false);
	}
}