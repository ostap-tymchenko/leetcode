use std::collections::HashSet;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = vec![];

    for word in strs {
        ret.push(vec![word]);
    }
    
    for a in 0..ret.len() {
        for b in 0..ret.len() {
            if Vec::from_iter(ret[a]).sort() == Vec::from_iter(ret[b]).sort() && a != b {
                ret[a].push(ret[b][0])

            }
        }
    }

    todo!()
}

fn str_vec_to_string_vec (strs: Vec<&str>) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    dbg!(group_anagrams(str_vec_to_string_vec(vec!["eat","tea","tan","ate","nat","bat"])));
}
