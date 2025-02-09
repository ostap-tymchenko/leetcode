// Given an integer array nums, return an integer array counts where counts[i] is the number of smaller elements to the right of nums[i].
//
// The problem is tagged with binary search

#[derive(Debug)]
struct BinaryTree<'a> {
    k: i32,
    l: Option<&'a BinaryTree<'a>>,
    r: Option<&'a BinaryTree<'a>>,
}

fn gen_binary_tree(mut nums: Vec<i32>) -> BinaryTree<'static> {
    let len = nums.len();
    if len <= 1{
        return BinaryTree {k: nums[0], l: None, r: None};
    } else {
        let mid = len / 2;

        let left = &nums[0..mid -1];
        let right = &nums[mid..len];

        return BinaryTree {k:mid, l:gen_binary_tree(left), r:gen_binary_tree(right)};
    }
}


// fn gen_binary_tree(mut nums: Vec<i32>) -> BinaryTree {
//     if nums.len() > 1 {
//         nums.sort();
//
//         let mid_i = nums.len() / 2;
//         let mid = nums[mid_i];
//
//         nums.remove(mid_i);
//
//         let binary_tree = BinaryTree {
//             k: mid,
//             l: Some(Box::new(gen_binary_tree(
//                 nums.clone().iter().filter(|x| **x < mid).collect(),
//             ))),
//             // r: Some(Box::new(gen_binary_tree(nums))),
//         };
//
//         return binary_tree;
//     } else {
//         let binary_tree = BinaryTree {
//             // leaf moment
//             k: nums[0],
//             l: None,
//             r: None,
//         };
//
//         return binary_tree;
//     }
// }

pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let tree = gen_binary_tree(nums);

    dbg!(tree);

    todo!();
}

fn main() {
    assert_eq!(count_smaller(vec![5, 2, 6, 1]), [2, 1, 1, 0]);
}
