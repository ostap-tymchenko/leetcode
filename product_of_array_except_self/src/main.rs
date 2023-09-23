// pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//     let len = nums.len();
//     let mut result: Vec<i32> = vec![1; len];
//     let mut before: i32;
//     let mut after: i32;
//
//     for a in 0..len {
//         before = nums[0..a].iter().product();
//         after = nums[a + 1..].iter().product();
//         result[a] = before * after;
//     }
//
//     result
// }

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut result: Vec<i32> = vec![1; len];
    let mut before: i32;
    let mut after: i32;

    for a in 0..len {
        before = nums[0..a].iter().product();
        after = nums[a + 1..].iter().product();
        result[a] = before * after;
    }

    result
}

fn main() {
    dbg!(product_except_self(vec![1,2,3,4]));
}
