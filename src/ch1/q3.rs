#[cfg(test)]
fn urlify (input_string: &str, true_length:i32) -> String {
    let mut return_string = String::with_capacity(input_string.len());
    for (i, c) in input_string.chars().enumerate() {
        if i == true_length as usize {
            break;
        } else {
            if c == ' ' {
                return_string.push_str("%20");
            } else {
                return_string.push(c);
            }
        }
    }
    if return_string.len() != input_string.len() {
        panic!("{message}", message="Wrong inputs.");
    }
    return return_string;
}

#[cfg(test)]
// This is the methid described in the book.
fn urlify_reverse(input_string: &str, true_length:i32) -> String {
    let mut space_count: i32 = 0;
    for (i, c) in input_string.chars().enumerate() {
        if i == true_length as usize {
            break;
        }
        if c == ' ' {
            space_count += 1;
        }
    }
    let mut runner: usize = true_length as usize + (space_count as usize * 2);
    let mut char_vec: Vec<char> = input_string.chars().collect();
    for i in (0..true_length).rev() {
        if char_vec[i as usize] == ' ' {
            char_vec[runner-1] = '0';
            char_vec[runner-2] = '2';
            char_vec[runner-3] = '%';
            runner -= 3;
        } else {
            char_vec[runner-1] = char_vec[i as usize];
            runner -= 1;
        }
    }
    let return_string: String = char_vec.into_iter().collect(); 
    return return_string;
}

#[cfg(test)]
mod q3_tests {
    use ch1::q3::urlify;
    use ch1::q3::urlify_reverse;

    #[test]
    fn urlify_test_1() {
        assert_eq!(urlify("Mr John Smith    ", 13), "Mr%20John%20Smith");
    }

    #[test]
    fn urlify_test_2() {
        assert_eq!(urlify("Mr John Smith       ", 14), "Mr%20John%20Smith%20");
    }

    #[test]
    fn urlify_test_3() {
        assert_eq!(urlify(" Mr John Smith      ", 14), "%20Mr%20John%20Smith");
    }

    #[should_panic]
    #[test]
    fn urlify_test_4() {
        assert_eq!(urlify(" Mr John Smith     ", 14), "%20Mr%20John%20Smith");
    }

    #[test]
    fn urlify_reverse_test_1() {
        assert_eq!(urlify_reverse("Mr John Smith    ", 13), "Mr%20John%20Smith");
    }

    #[test]
    fn urlify_reverse_test_2() {
        assert_eq!(urlify_reverse("Mr John Smith       ", 14), "Mr%20John%20Smith%20");
    }

    #[test]
    fn urlify_reverse_test_3() {
        assert_eq!(urlify_reverse(" Mr John Smith      ", 14), "%20Mr%20John%20Smith");
    }

    #[should_panic]
    #[test]
    fn urlify_reverse_test_4() {
        assert_eq!(urlify_reverse(" Mr John Smith     ", 14), "%20Mr%20John%20Smith");
    }        
}