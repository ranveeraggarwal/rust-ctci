#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
fn permutation (string_1: &str, string_2: &str) -> bool {
    let mut hashmap = HashMap::new();
    for character in string_1.chars() {
        let count = hashmap.entry(character).or_insert(0);
        *count += 1;
    }
    for character in string_2.chars() {
        let count = hashmap.entry(character).or_insert(-1);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }
    for val in hashmap.values() {
        if *val != 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod q2_tests {
    use ch1::q2::permutation;

    #[test]
    fn is_permutation() {
        assert!(permutation("ranveer", "veenrar"));
    }

    #[test]
    fn is_not_permutation() {
        assert_eq!(permutation("ranveer", "sanveer"), false);
    }

    #[test]
    fn different_sized_permutation() {
        assert_eq!(permutation("ranveer", "ranveers"), false);   
    }

    #[test]
    fn another_different_sized_permutation() {
        assert_eq!(permutation("ranveer", "ranv"), false);   
    }

    #[test]
    fn one_empty_string_permutation() {
        assert_eq!(permutation("ranveer", ""), false);   
    }

    #[test]
    fn both_empty_string_permutation() {
        assert!(permutation("", ""));   
    }

    #[test]
    fn same_characters_permutation() {
        assert!(permutation("aaaa", "aaaa"));   
    }    
}