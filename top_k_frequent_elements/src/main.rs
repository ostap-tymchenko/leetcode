use std::collections::{HashMap, BTreeMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // let mut learderboard = Vec::new();
    let mut map = HashMap::new();

    for dgt in 0..9 {
        let score = nums.iter().filter(|&x| *x == dgt).count() as i32;
        if score > 0 {
            println!("score: {score}, dgt: {dgt}");
            map.insert(score, dgt);
        }
        // learderboard.push(dgt);
    }

    dbg!(map.keys());
    todo!();
    
    // learderboard.sort();
    // learderboard.drain(k as usize..);
    // learderboard
}

fn main() {
    dbg!(top_k_frequent(vec![1,1,1,2,2,3], 2));
}
