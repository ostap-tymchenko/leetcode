pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn usize_to_i32(n:usize) -> i32  {
        n.try_into().expect("failed conversion")
    }

    for (index1, num1) in nums.iter().enumerate() {
        for (index2, num2) in nums.iter().enumerate() {
            if (num1 + num2) == target && index1 != index2 {
                return vec![usize_to_i32(index1), usize_to_i32(index2)];
            }
        }
    }

    panic!("could not create target ({target}) from nums ({nums:?})");
}
