fn main() {
    let ans = Solution::hammingWeight(1111);
    println!("ans: {:?}", ans);
}

struct Solution {}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}
