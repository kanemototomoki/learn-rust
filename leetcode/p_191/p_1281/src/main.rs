fn main() {
    let ans = Solution::subtract_product_and_sum(111);
    println!("ans: {}", ans);
}

struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let s = n.to_string();
        let multi = s
            .chars()
            .into_iter()
            .fold(1, |acc, x| acc * x.to_digit(10).unwrap() as i32);

        let sum = s
            .chars()
            .into_iter()
            .fold(0, |acc, x| acc + x.to_digit(10).unwrap() as i32);

        multi - sum
    }
}
