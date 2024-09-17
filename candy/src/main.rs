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

fn i32_to_usize(i: i32) -> usize {
    if i <= 0 {
        0
    } else {
        i as usize
    }
}

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candy = 0;

    for (child_num, rate) in ratings.iter().enumerate() {
        let mut number_of_candy_for_this_child = 0;

        let left_child_rate: &i32 = &ratings.get(i32_to_usize(child_num as i32 - 1)).unwrap_or(&0);
        let right_child_rate: &i32 = &ratings.get(i32_to_usize(child_num as i32 + 1)).unwrap_or(&0);

        let mut top = rate;
        let mut bottom = rate;

        if left_child_rate > top {
            top = left_child_rate;
        } else if left_child_rate < top {
            bottom = left_child_rate;
        }

        if right_child_rate > top {
            top = right_child_rate;
        } else if left_child_rate < top {
            bottom =right_child_rate;
        }

        if rate == top {
            // Heighest case
        } else if  rate == bottom {
            // Lowest case
            number_of_candy_for_this_child = 1;
        }

    }

    candy
}
