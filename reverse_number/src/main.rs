// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
//
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

use std::ops::Mul;

pub fn reverse(x: i32) -> i32 {
    if x < 0 {
        x.to_string()
            .replace('-', "")
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
            .mul(-1)

    } else {
        x.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
    }
}

fn main() {
    // dbg!(reverse_positive(123));
    dbg!(reverse(500));
    dbg!(reverse(-123));
    dbg!(reverse(-32));
    // dbg!(reverse(-9));
    // reverse(00);
    // reverse(7541293);
}
