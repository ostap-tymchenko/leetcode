pub fn merge_alternately(word1: String, word2: String) -> String {
    let max_len;

    if word1.len() > word2.len() {
        max_len = word1.len();
    } else {
        max_len = word2.len();
    }

    let mut v: Vec<char> = Vec::new();

    for current in 0..max_len {
        // Even case
        if current % 2 == 0 {
             match word1.chars().nth(current) {
                 Some(c) => { 
                     v.insert(current, c);
                     break;
                 },
                 None => v.insert(current, word2.chars().nth(current).expect("Both not max size")),
             };
        } else {
        // Odd case

        match word2.chars().nth(current) {
             Some(c) => { 
                 v.insert(current, c);
                 break;
             },
             None => v.insert(current, word1.chars().nth(current).expect("Both not max size")),
            };
        };
    };

    dbg!(v);
    
    // println!("{max_len}");

    todo!();
}

fn do_nothing() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_alternately() {
        assert_eq!(merge_alternately("abc".to_string(), "pqr".to_string()), "apbqcr");
    }
}

fn main() {
    merge_alternately("abc".to_string(), "pqr".to_string());
}
