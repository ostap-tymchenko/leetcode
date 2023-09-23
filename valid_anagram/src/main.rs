pub fn is_anagram(s: String, t: String) -> bool {
    let mut a = Vec::from_iter(s.chars());
    let mut b = Vec::from_iter(t.chars());
    a.sort();
    b.sort();
    a == b
}

fn main() {
    println!("Hello, world!");
    dbg!(is_anagram("rat".to_string(), "car".to_string()));
}
