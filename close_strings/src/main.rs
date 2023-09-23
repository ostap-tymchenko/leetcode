use std::collections::HashSet;

// Operation 1: Swap any two existing characters.
// For example, abcde -> aecdb
// Operation 2: Transform every occurrence of one existing
// character into another existing character, and do the
// same with the other character.
// For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)

pub fn close_strings(word1: String, word2: String) -> bool {
    let mut w1_vec: Vec<char> = Vec::from_iter(word1.chars());
    let mut w2_vec: Vec<char> = Vec::from_iter(word2.chars());

    w1_vec.sort();
    w2_vec.sort();

    dbg!(&w1_vec, &w2_vec);

    if word1.len() == word2.len() && w1_vec == w2_vec {
        return true;
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_1657_examples() {
        assert_eq!(close_strings("abc".to_string(), "bca".to_string()), true);
        assert_eq!(close_strings("a".to_string(), "aa".to_string()), false);
        assert_eq!(
            close_strings("cabbba".to_string(), "abbccc".to_string()),
            true
        );
    }

    #[test]
    fn leetcode_1657_examples_bug_1() {
        assert_eq!(close_strings("cabbba".to_string(), "aabbss".to_string()), false);
    }
}
