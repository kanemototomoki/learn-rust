fn main() {
    let s = "MCMXCIV".to_string();
    let ans = Solution::roman_to_int(s);
    println!("ans: {}", ans);
}

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;

        for c in s.chars() {
            match c {
                'I' => sum += 1,
                'V' => sum += 5,
                'X' => sum += 10,
                'L' => sum += 50,
                'C' => sum += 100,
                'D' => sum += 500,
                'M' => sum += 1000,
                _ => sum += 0,
            };
        }

        if s.contains("IV") | s.contains("IX") {
            sum -= 2;
        }
        if s.contains("XL") | s.contains("XC") {
            sum -= 20;
        }
        if s.contains("CD") | s.contains("CM") {
            sum -= 200;
        }
        sum
    }
}
