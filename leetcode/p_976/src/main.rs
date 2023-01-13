fn main() {
    let ans = Solution::largest_perimeter(vec![2, 1, 2]);
    println!("ans: {:?}", ans);
}

struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut n = nums.clone();
        n.sort();
        n.reverse();
        for i in 0..(n.len() - 2) {
            if n[i] < n[i + 1] + n[i + 2] {
                return n[i] + n[i + 1] + n[i + 2];
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let res = Solution::largest_perimeter(vec![2, 1, 2]);
        assert_eq!(res, 5);
    }

    #[test]
    fn case2() {
        let res = Solution::largest_perimeter(vec![1, 2, 1, 10]);
        assert_eq!(res, 0);
    }
}
