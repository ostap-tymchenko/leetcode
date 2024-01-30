use std::collections::HashSet;

fn get_first_instance_of_char_in_string(s: &String, c:char) -> Option<usize> {
    for (index, letter) in s.chars().enumerate() {
        if letter == c {
            return Some(index);
        }
    }

    return None;
}

pub fn min_window(s: String, t: String) -> String {
    // let t_alphabet = HashSet::from_iter(t.chars());
    let mut first_instances = vec![];

    for letter in t.chars() {
        first_instances.push(get_first_instance_of_char_in_string(&s, letter));
    }
    
    dbg!(first_instances);
    todo!();
}

fn main() {
    dbg!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()));
}
