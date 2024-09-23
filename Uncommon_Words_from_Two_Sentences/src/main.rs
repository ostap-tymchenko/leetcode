use std::collections::HashSet;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut set: HashSet<&str> = HashSet::new();

    for word in s1.split_whitespace() {
        println!("{word}");
        set.insert(word);
    }

    for word in s2.split_whitespace() {
        println!("{word}");
        set.insert(word);
    }

    dbg!(set);

}

fn main() {
    let s1 = String::from("this apple is sweet");
    let s2 = String::from("this apple is sour");
    dbg!(uncommon_from_sentences(s1, s2));
}
