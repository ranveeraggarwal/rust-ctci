#[cfg(test)]
fn is_rotation (s1: &str, s2: &str) -> bool {
	let double_string: String = format!("{}{}", s2, s2);
	double_string.contains(s1)
}

#[cfg(test)]
mod q9_tests {
	use ch1::q9::is_rotation;

	#[test]
	fn is_rotation_waterbottle() {
		assert!(is_rotation("waterbottle", "erbottlewat"));
	}

	#[test]
	fn is_rotation_abc() {
		assert!(is_rotation("abc", "bca"));
	}

	#[test]
	fn is_rotation_a() {
		assert!(is_rotation("a", "a"));
	}

	#[test]
	fn is_rotation_empty() {
		assert!(is_rotation("", ""));
	}

	#[test]
	fn is_rotation_false() {
		assert_eq!(is_rotation("false", "esfal"), false);
	}
}