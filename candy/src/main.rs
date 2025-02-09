use std::fs;
    use std::path::Path;

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";

fn i32_to_usize(i: i32) -> usize {
    if i <= 0 {
        0
    } else {
        i as usize
    }
}

fn read_data_from_name(file_name: &str) -> String {
    let path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    fs::read_to_string(Path::new(&path))
        .unwrap_or_else(|_| panic!("data parse fail, looking for {file_name}"))
}

fn main() {
    let mut example = Vec::new();

    for word in read_data_from_name("large-data.txt").split_whitespace() {
        example.push(word.parse().unwrap());
    }

    // let example = vec![0,1,2,3,9,4,1];

    dbg!(candy(example));
}

pub fn candy(ratings: Vec<i32>) -> i32 {
    double_sided_increment_strategy(ratings)
}

fn double_sided_increment_strategy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];

    for (index, rate) in ratings.iter().enumerate() {
        if index != 0 {
            if *rate > ratings[index - 1] {
                if candies[index] <= candies[index - 1] {
                    candies[index] = candies[index - 1] + 1;
                }
            }
        }
    }

    for (mut index, rate) in ratings.iter().rev().enumerate() {
        // dbg!(index);
        index = ratings.len() - index - 1;

        if index != ratings.len() - 1 {
            // dbg!(index);
            // dbg!(ratings.len());
            if *rate > ratings[index + 1] {
                if candies[index] <= candies[index + 1] {
                    candies[index] = candies[index + 1] + 1;
                }
            }
        }
    }

    // dbg!(&ratings);
    // dbg!(&candies);

    candies.iter().sum()
}

// fn layered_increase_strategy(ratings: Vec<i32>) -> i32 {
//     let mut candies = vec![1; ratings.len()];
//
//     'outer: loop {
//         let mut edited = false;
//
//         for (index, rate) in ratings.iter().enumerate() {
//                 let l_rate: &i32 = ratings.get(i32_to_usize(index as i32 - 1)).unwrap_or(&-1);
//                 let r_rate: &i32 = ratings.get(i32_to_usize(index as i32 + 1)).unwrap_or(&-1);
//
//                 if rate > l_rate && *l_rate != -1 && candies[index - 1] >= candies[index] {
//                     candies[index] += 1;
//                     edited = true;
//                 }
//
//                 if rate > r_rate && *r_rate != -1 && candies[index + 1] >= candies[index] {
//                     candies[index] += 1;
//                     edited = true;
//                 }
//         }
//
//         if !edited {
//             break 'outer;
//         }
//     }
//
//     candies.iter().sum()
// }

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
        assert_eq!(candy(vec![29, 51, 87, 87, 72, 12]), 12);
    }
}
