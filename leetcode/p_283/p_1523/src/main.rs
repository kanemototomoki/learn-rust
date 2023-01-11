fn main() {
    let ans = Solution::count_odds(2, 6);
    println!("ans: {}", ans);
}

struct Solution {}
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let sum = high - low + 1;
        dbg!(sum);
        if sum % 2 == 0 {
            return sum / 2;
        } else {
            dbg!((sum - 1) % 2 == 0);
            if (sum - 1) % 2 == 0 {
                if low % 2 == 0 || high % 2 == 0 {
                    return (sum - 1) / 2;
                } else {
                    return (sum - 1) / 2 + 1;
                }
            } else {
                return (sum - 2) / 2;
            }
        }
    }
}
