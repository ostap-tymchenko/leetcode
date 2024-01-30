pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().chain(nums.iter()).map(|&x| x).collect::<Vec<i32>>()
}

fn main() {
    dbg!(get_concatenation(vec![1,2,3]));
}
