use structures::singlies::List;
use structures::singlies::Iter;

// Returns 0 if k > list size
fn kthlast(list: &List<i32>, k: i32) -> i32 {
	let mut iter_ahead = list.iter();
	let mut iter_behind = list.iter();
	let mut pos = k;
	let mut current: i32 = match list.head() {
		Some(value) => *value,
		None => 0,
	};
	while pos > 0 {
		iter_ahead.next();
		pos -= 1;
	}
	loop {
		match iter_behind.next() {
			Some(value) => current = *value,
			None => break,
		};
		match iter_ahead.next() {
			Some(value) => *value,
			None => break,
		};
	}
	current
}

#[cfg(test)]
mod q2_tests {
	use structures::singlies::List;
	use ch2::q2::kthlast;

	#[test]
	fn first_test() {
		let list = List::new();
		let list = list.append(3).append(2).append(1);
		let mut k = 2;
		assert_eq!(kthlast(&list, k), 2);
	}

	#[test]
	fn second_test() {
		let list = List::new();
		let list = list.append(2).append(5).append(6).append(3).append(7);
		let mut k = 4;
		assert_eq!(kthlast(&list, k), 3);
	}

	#[test]
	fn fourth_test() {
		let list = List::new();
		let mut k = 1;
		assert_eq!(kthlast(&list, k), 0);
	}
}