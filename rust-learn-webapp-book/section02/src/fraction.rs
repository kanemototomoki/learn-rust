use std::cmp::{Eq, PartialEq};
use std::fmt::{self, Display};
use std::ops::Add;

// Fraction 分数
#[derive(Debug, PartialEq, Eq)]
struct Fraction(u32, u32);

impl Fraction {
    // numerator: 分子 denominator: 分母
    fn new(numerator: u32, denominator: u32) -> Self {
        let gcf_value = Self::gcf(numerator, denominator);
        Self(numerator / gcf_value, denominator / gcf_value)
    }

    fn gcf(v1: u32, v2: u32) -> u32 {
        // ユークリッドの互除法
        let (mut a, mut b) = if v1 > v2 { (v1, v2) } else { (v2, v1) };

        let mut r = a % b;
        while r != 0 {
            a = b;
            b = r;
            r = a % b;
        }
        b
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", &self.0, &self.1)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // 最小公約数
        let lcm = self.1 / Self::gcf(self.1, other.1) * other.1;

        let a = self.0 * (lcm / self.1);
        let b = other.0 * (lcm / other.1);
        Fraction::new(a + b, lcm)
    }
}

pub fn run() {
    let a = Fraction::new(8, 18);
    let b = Fraction::new(1, 2);
    println!("fraction a: {}, fraction b: {}", a, b);
    println!("fraction a + b: {}", a + b);
}
