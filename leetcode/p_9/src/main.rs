fn main() {
    let ans = Solution::is_palindrome(-121);
    println!("answer: {}", ans);
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_rev: Result<i32, _> = x.to_string().chars().rev().collect::<String>().parse();
        if x_rev.is_ok() {
            return x_rev.unwrap() == x;
        } else {
            return false;
        }
    }
}
