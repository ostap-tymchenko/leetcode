pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    for _ in 0..k {

        let min = nums.iter().min().unwrap();
        let min_index = nums.iter().position(|x| x==min ).unwrap();

        nums[min_index] = nums[min_index] * multiplier

    }

    let mut ret_nums: Vec<i32> = Vec::with_capacity(nums.len());

    for (index, _) in nums.iter().enumerate() {
        ret_nums.push(nums[index] % 1000000007);
    }

    ret_nums
}

fn main() {
    dbg!(get_final_state(vec![2,1,3,5,6], 5, 2));
    dbg!(assert_eq!(get_final_state(vec![100000,2000], 2, 1000000)), [999999307,999999993]
    // dbg!(get_final_state(vec![66307295,441787703,589039035,322281864], 900900704, 641725));
}
