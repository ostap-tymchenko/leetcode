pub fn is_palindrome(s: String) -> bool {
    let cleaned = s.chars().filter(|c| c.is_alphabetic() || c.is_numeric()).collect::<String>().to_lowercase();
    let reversed = cleaned.chars().rev().collect::<String>();
    cleaned == reversed
}

fn main() {
    dbg!(is_palindrome("0".to_string()));
}
