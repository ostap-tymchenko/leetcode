use std::collections::HashSet;
use crate::library::*;

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let rows = [
        str_to_hashset("qwertyuiop"),
        str_to_hashset("asdfghjkl"),
        str_to_hashset("zxcvbnm"),
    ];

    fn can_make_word_from_row(row: &HashSet<char>, word: &str) -> bool {
        for letter in word.chars() {
            if !row.contains(&letter.to_ascii_lowercase()) {
                return false;
            }
        }

        true
    }

    let mut buff: Vec<String> = vec![];

    for word in &words {
        for row in &rows {
            if can_make_word_from_row(row, word) {
                buff.push(word.to_string());
            }
        }
    }

    buff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_500_examples() {
        assert_eq!(
            find_words(str_vec_to_string_vec(vec!["Hello", "Alaska", "Dad", "Peace"])).sort(),
            str_vec_to_string_vec(vec!["Alaska", "Dad"]).sort()
        );

        assert_eq!(
            find_words(str_vec_to_string_vec(vec!["omk"])).sort(),
            str_vec_to_string_vec(vec![]).sort()
        );

        assert_eq!(
            find_words(str_vec_to_string_vec(vec!["adsdf", "sfd"])).sort(),
            str_vec_to_string_vec(vec!["adsdf", "sfd"]).sort()
        );
    }
}
