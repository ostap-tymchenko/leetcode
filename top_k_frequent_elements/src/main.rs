// use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Element(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Frequency(u32);

use std::cmp::Ordering;

trait Sorted<T> {
    fn sorted(self) -> Vec<T> where T: Ord;
}

impl<T: Ord> Sorted<T> for Iterator<Item = T> {
    fn sorted(self) -> Vec<T> {
        let mut vec = Vec::new();
        for item in self {
            vec.push(item);
        }
        vec.sort();
        vec
    }
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut leaderboard: HashMap<Element, Frequency> = Default::default();

    for k in nums.iter().map(|x| Element(*x)) {
        leaderboard.entry(k).or_insert(Frequency(0)).0 += 1;
    }

    leaderboard
        .iter()
        .map(|(k, v)| (v, k))
        .sorted()
        .rev()
        .map(|(_, v)| v.0)
        .collect::<Vec<i32>>()
        .drain(..k as usize)
        .collect()
}

fn main() {
    dbg!(top_k_frequent(vec![1, 1, 1, 2, 2, 3, 4, 4, 4, 4, 4], 3));
}
