pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
    let mut top_chain = 0;
    let mut chain = 1;

    nums.sort();
    nums.dedup();

    let mut last = -1000000001;

    for num in nums {
        if num - 1 == last {
            chain += 1;
        } else {
            chain = 1;
        }

        if chain > top_chain {
            top_chain = chain
        }

        last = num;
    }

    top_chain
}

fn main() {
    dbg!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]));
    dbg!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    dbg!(longest_consecutive(vec![1,2,0,1]));
    dbg!(longest_consecutive(vec![1]));
}
