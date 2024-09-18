// There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
//
// You are giving candies to these children subjected to the following requirements:
//
// Each child must have at least one candy.
// Children with a higher rating get more candies than their neighbors.
// Return the minimum number of candies you need to have to distribute the candies to the children.

use std::usize;

fn i32_to_usize(i: i32) -> usize {
    if i <= 0 {
        0
    } else {
        i as usize
    }
}

fn main() {
    let example_1 = vec![1, 5, 2,];
    dbg!(candy(example_1));
}

// fn determine_current_child_candy(ratings: Vec<i32>, child_num: usize) -> i32 {
//     let mut give_candy = 0;
//
//     let rate = ratings[child_num];
//     dbg!(rate);
//
//     let left_child_rate: &i32 = &ratings
//         .get(i32_to_usize(child_num as i32 - 1))
//         .unwrap_or(&0);
//     let right_child_rate: &i32 = &ratings
//         .get(i32_to_usize(child_num as i32 + 1))
//         .unwrap_or(&0);
//
//     if rate == 0 {
//         give_candy = 1;
//     } 
//
//     println!("START rate:{rate}");
//
//     if rate > *left_child_rate && rate > *right_child_rate && child_num != 0 && child_num + 1 != ratings.len() {
//         if *left_child_rate == 0 {
//             println!("END rate:2, A");
//             return 2;
//         } else {
//             println!("END rate:{}, B", left_child_rate + 1);
//             return left_child_rate + 1;
//         }
//     } else if rate > *left_child_rate && child_num != 0 {
//         if *left_child_rate == 0 {
//             println!("END rate:2, C");
//             return 2;
//         } else {
//             println!("END rate:{}, D", left_child_rate +1);
//             return left_child_rate + 1;
//         }
//     } else if rate > *right_child_rate && child_num + 1 != ratings.len() {
//         dbg!(ratings.len(), child_num);
//
//         if *right_child_rate == 0 {
//             println!("END rate:2, E");
//             return 2;
//         } else {
//             println!("END rate:{}, F", right_child_rate +1);
//             return right_child_rate + 1;
//         }
//     } else {
//         println!("END rate:1, G");
//         return 1;
//     }
// }

fn layered_increase_strategy(ratings: Vec<i32>) -> i32 {

    let candy_array = vec![1;ratings.len()];

    dbg!(&candy_array);

    for (child_num, current_rate) in ratings.iter().enumerate() {

        let left_rate: &i32 = &ratings
            .get(i32_to_usize(child_num as i32 - 1))
            .unwrap_or(&-1);

        let right_rate: &i32 = &ratings
            .get(i32_to_usize(child_num as i32 + 1))
            .unwrap_or(&-1);
        }

    candy_array.iter().sum()
}

pub fn candy(ratings: Vec<i32>) -> i32 {
    layered_increase_strategy(ratings)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_candy() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn test_candy_2() {
        assert_eq!(candy(vec![1, 2, 2]), 4);
        //                    1  2, 1
    }

    #[test]
    fn test_candy_3() {
        assert_eq!(candy(vec![29,51,87,87,72,12]), 12);
    }
}
