use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use std::fs;
use std::path::Path;
use std::usize;

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
        .expect(&format!("data parse fail, looking for {file_name}"))
}

// fn main() {
//     let mut example = Vec::new();
//
//     for word in read_data_from_name("large-data.txt").split_whitespace() {
//         example.push(word.parse().unwrap());
//     }
//
//     dbg!(candy(example));
// }

pub fn candy(ratings: Vec<i32>) -> i32 {
    layered_increase_strategy_with_ignorelist(ratings)
}

fn layered_increase_strategy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];

    // let mut ignore_list = vec![false; ratings.len()];

    let mut x = 0;
    'outer: loop {
        x += 1;
        let mut edited = false;

        for (index, rate) in ratings.iter().enumerate() {
            // if !ignore_list[index] {

            let l_rate: &i32 = &ratings.get(i32_to_usize(index as i32 - 1)).unwrap_or(&-1);
            let r_rate: &i32 = &ratings.get(i32_to_usize(index as i32 + 1)).unwrap_or(&-1);

            if rate > l_rate && *l_rate != -1 && candies[index - 1] >= candies[index] {
                candies[index] += 1;
                edited = true;
            }

            if rate > r_rate && *r_rate != -1 && candies[index + 1] >= candies[index] {
                candies[index] += 1;
                edited = true;
            }

            // if edited == false {
            //     ignore_list[index] = true;
            // }
            // }
        }

        if edited == false {
            break 'outer;
        }
    }
    // dbg!(x);

    candies.iter().sum()
}

fn benchmark(c: &mut Criterion) {
    let mut example = Vec::new();

    for word in read_data_from_name("large-data.txt").split_whitespace() {
        example.push(word.parse().unwrap());
    }

    c.bench_function("layered_increase_strategy_with_ignorelist", |b| {
        b.iter(|| black_box(candy(example.clone())))
    });
    c.bench_function("layered_increase_strategy", |b| {
        b.iter(|| black_box(candy(example.clone())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
