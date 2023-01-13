fn main() {
    let ans = Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]);
    println!("ans: {}", ans);
}

struct Solution {}
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let xy_vec = vec![x, y];
        let mut p: (i32, i32) = (i32::MAX, 0);

        for (i, v) in points.into_iter().enumerate() {
            if x != v[0] && y != v[1] {
                continue;
            }
            if xy_vec == v {
                return i as i32;
            }
            let n = (x - v[0]).abs() + (y - v[1]).abs();
            if n != 0 && p.0 > n {
                p = (n, i as i32);
            }
        }

        if p.0 == i32::MAX && p.1 == 0 {
            return -1;
        }
        p.1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let res = Solution::nearest_valid_point(
            3,
            4,
            vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]],
        );
        assert_eq!(res, 2);
    }

    #[test]
    fn case2() {
        let res = Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]);
        assert_eq!(res, 0);
    }

    #[test]
    fn case3() {
        let res = Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]);
        assert_eq!(res, -1);
    }

    #[test]
    fn case4() {
        let res = Solution::nearest_valid_point(1, 1, vec![vec![1, 2], vec![3, 3], vec![3, 3]]);
        assert_eq!(res, 0);
    }

    #[test]
    fn case5() {
        let res = Solution::nearest_valid_point(1, 1, vec![vec![1, 2], vec![2, 1], vec![1, 2], vec![1, 1]]);
        assert_eq!(res, 3);
    }
}
