use std::{collections::HashMap, str::Chars};

// extern crate itertools;
// use crate::itertools::Itertools;

// #[cfg(debug_assertions)]
// use std::any::type_name;
// use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Alphabet(Vec<char>);

trait Sorted {
    fn sorted(self) -> std::vec::IntoIter<char>;
}

impl Sorted for Chars<'_> {
    fn sorted(self) -> std::vec::IntoIter<char> {
        let mut v = Vec::from_iter(self);
        v.sort();
        v.into_iter()
    }
}

impl Alphabet {
    pub fn construct(letters: Chars) -> Alphabet {
        Alphabet(Vec::from_iter(letters.sorted()))
    }
}

// #[cfg(debug_assertions)]
// fn deduplicate_map_of_vectors_of_strings(map: &mut HashMap<Alphabet, Vec<String>>) {
//     for (alphabet, strings) in map.iter_mut() {
//         let mut unique_strings: HashSet<String> = HashSet::new();
//         strings.retain(|string| unique_strings.insert(string.clone()));
//     }
// }

pub fn group_anagrams(strs: Vec<String>y -> Vec<Vec<String>> {
    let mut anagrams: HashMap<Alphabet, Vec<String>> = Default::default();

    for word in strs {
        let alpha = Alphabet::construct(word.chars());
        anagrams.entry(alpha).or_insert(vec![]).push(word)
    }

    // dbg!(anagrams.into_values().for_each(|x| {dbg!(type_of(x));}));
    anagrams.values().cloned().collect::<Vec<Vec<String>>>()
    // dbg!(type_of(a.clone()), a);
}

fn str_vec_to_string_vec (strs: Vec<&str>) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    dbg!(group_anagrams(str_vec_to_string_vec(vec!["eat","tea","tan","ate","nat","bat"])));
}
