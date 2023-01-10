fn main() {
    let ans = Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
    println!("ans: {}", ans);
}

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}
