// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     fn usize_to_i32(n:usize) -> i32  {
//         n.try_into().expect("failed conversion")
//     }
//
//     for (index1, num1) in nums.iter().enumerate() {
//         for (index2, num2) in nums.iter().enumerate() {
//             if (num1 + num2) == target && index1 != index2 {
//                 return vec![usize_to_i32(index1), usize_to_i32(index2)];
//             }
//         }
//     }
//
//     panic!("could not create target ({target}) from nums ({nums:?})");
// }

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    
    for (index, consider) in nums.iter().enumerate() {
        map.insert(consider, index);
        let get_index = map.get(&(target - consider));
        if get_index.is_some() && get_index.unwrap() != &index {
            return vec![index as i32, *get_index.unwrap() as i32];
        }
    }

    unreachable!();
}

fn main() {
    dbg!(two_sum(vec![1, 3 ,2 , 4], 6));

}
