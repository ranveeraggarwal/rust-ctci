#[cfg(test)]
use std::collections::HashSet;
#[cfg(test)]
use std::collections::LinkedList;

#[cfg(test)]
fn remove_duplicates_bad(list: &LinkedList<u32>) -> LinkedList<u32> {
	let mut set = HashSet::new();
	let mut return_list = LinkedList::new();

	for node in list {
		if !set.contains(node) {
			return_list.push_back(*node);
		} 
		set.insert(node);
	}

	return return_list;
}

#[cfg(test)]
mod q1_tests {
	use ch2::q1::remove_duplicates_bad;
	use std::collections::LinkedList;

	#[test]
	fn remove_duplicates_bad_random() {
		let mut list_1 = LinkedList::new();
		let mut list_2 = LinkedList::new();
		list_1.push_back(1);
		list_1.push_back(2);
		list_1.push_back(3);
		list_1.push_back(2);
		list_2.push_back(1);
		list_2.push_back(2);
		list_2.push_back(3);
		let unique_list_1 = remove_duplicates_bad(&list_1);
		assert_eq!(list_2, unique_list_1);
	}

	#[test]
	fn remove_duplicates_bad_singly() {
		let mut list_1 = LinkedList::new();
		let mut list_2 = LinkedList::new();
		list_1.push_back(1);
		list_2.push_back(1);
		let unique_list_1 = remove_duplicates_bad(&list_1);
		assert_eq!(list_2, unique_list_1);
	}

	#[test]
	fn remove_duplicates_bad_cascading() {
		let mut list_1 = LinkedList::new();
		let mut list_2 = LinkedList::new();
		list_1.push_back(1);
		list_1.push_back(1);
		list_1.push_back(1);
		list_1.push_back(1);
		list_2.push_back(1);
		let unique_list_1 = remove_duplicates_bad(&list_1);
		assert_eq!(list_2, unique_list_1);
	}
}