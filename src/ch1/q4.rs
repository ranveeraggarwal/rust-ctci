#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
// Does not handle palindrome_permutation("Aza") -> true
// This is because in unicode, there can be ambiguity in converting chars to uppercase. 
fn palindrome_permutation(input_string: &str) -> bool {
    let mut hashmap = HashMap::new();
    for character in input_string.chars() {
        if character != ' ' {
            let count = hashmap.entry(character).or_insert(0);
            *count += 1;
        }
    }
    let mut odd_count: i32 = 0;
    for val in hashmap.values() {
        if *val % 2 == 1 {
            odd_count += 1;
            if odd_count > 1 {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod q4_tests {
    use ch1::q4::palindrome_permutation;

    #[test]
    fn oll_is_palindrome_permutation() {
        assert!(palindrome_permutation("oll"));
    }

    #[test]
    fn ooolll_is_not_palindrome_permutation() {
        assert_eq!(palindrome_permutation("ooolll"), false);
    }

    #[test]
    fn a_string_is_not_palindrome_permutation() {
        assert_eq!(palindrome_permutation("so patient a nurse to nurse a patient so"), false);
    }    

    #[test]
    fn a_string_is_palindrome_permutation() {
        assert!(palindrome_permutation("R acecaR"));
    }

    #[test]
    fn a_unicode_string_is_palindrome_permutation() {
        assert!(palindrome_permutation("ʥቕ ʥቕ"));
    }    
}