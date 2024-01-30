pub fn max_area(height: Vec<i32>) -> i32 {
    let mut top_volume = 0;
    let len = height.len();

    // let mid = len / 2;
    for a in 0..len {
        for b in (0..len).rev() {
            let height_of_index_a = height[a];
            let height_of_index_b = height[b];

            let low;

            if height_of_index_b < height_of_index_a {
                low = height_of_index_b;
            } else {
                low = height_of_index_a;
            }

            let vol = (b as i32 - (a as i32).abs()) * low;

            println!("a index = {a}, a val = {height_of_index_a}, b index = {b}, b val = {height_of_index_b}");
            println!("low:{low} a:{a} b:{b} (b:{b}-a:{a}).abs()={} *=low:{low} == {vol}", b.abs_diff(a));
            println!();

            if vol > top_volume {
                top_volume = vol;
            }
        }
    }

    top_volume
}

fn main() {
    dbg!(max_area(vec![2, 3, 4, 5, 18, 17, 6]));

    // dbg!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    // assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
