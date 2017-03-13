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
mod q3_tests {
    use q3::urlify;

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

    #[test]
    #[should_panic]
    fn urlify_test_4() {
        assert_eq!(urlify(" Mr John Smith     ", 14), "%20Mr%20John%20Smith");
    }    
}