fn main() {
    let mut vec = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut vec);
    println!("ans: {:?}", vec);
}

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&x| x != 0);
        for _ in nums.len()..len {
            nums.push(0);
        }
    }
}
