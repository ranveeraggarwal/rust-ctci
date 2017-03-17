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
    fn permutation_ranveer() {
        assert!(permutation("ranveer", "veenrar"));
    }

    #[test]
    fn permutation_not_ranveer() {
        assert_eq!(permutation("ranveer", "sanveer"), false);
    }

    #[test]
    fn permutation_different_sized() {
        assert_eq!(permutation("ranveer", "ranveers"), false);   
    }

    #[test]
    fn permutation_another_different_sized() {
        assert_eq!(permutation("ranveer", "ranv"), false);   
    }

    #[test]
    fn permutation_one_empty_string() {
        assert_eq!(permutation("ranveer", ""), false);   
    }

    #[test]
    fn permutation_both_empty_string() {
        assert!(permutation("", ""));   
    }

    #[test]
    fn permutation_same_characters() {
        assert!(permutation("aaaa", "aaaa"));   
    }    
}