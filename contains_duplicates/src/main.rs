// pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//     let a: HashSet<&i32> = HashSet::from_iter(nums.iter());
//     if a.len() == nums.len() {
//         false
//     } else {
//         true
//     }
// }

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut cloned = nums.clone();
    cloned.sort();
    
    let mut last = cloned.first().unwrap() -1;
    for num in cloned {
        if num > last {
            last = num
        } else {
            return true;
        }
    }

    false
}

fn main() {
    // dbg!(contains_duplicate(vec![1,2,3,4,4,4,5,6,7]));
    // dbg!(contains_duplicate(vec![1,2,3,4,5,6,7]));
    // dbg!(contains_duplicate(vec![1,3,4,3,6,7]));
    dbg!(contains_duplicate(vec![1, 2, 3, 1]));
}
