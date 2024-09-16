// There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
//
// You are giving candies to these children subjected to the following requirements:
//
// Each child must have at least one candy.
// Children with a higher rating get more candies than their neighbors.
// Return the minimum number of candies you need to have to distribute the candies to the children.

fn main() {
    let example_1 = vec![1, 0, 2];
    dbg!(candy(example_1));
}

fn usize_to_i32(u: usize) -> i32 {
    u as i32
}

fn i32_to_usize(i: i32) -> usize {
    match i {
        (i <= 0) => 0;
        _ => i as usize;
    }
}

pub fn candy(ratings: Vec<i32>) -> i32 {
    for child in ratings {
        let left_child: i32 = ratings[child as i32 - 1];
        let right_child: i32 = ratings[child as i32 + 1];

        dbg!(child, left_child, right_child);
    }

    todo!();
}
