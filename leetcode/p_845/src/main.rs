fn main() {
    let ans = longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]);

    println!("ans: {}", ans);
}

pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let max = arr.iter().max().unwrap();
    let top = arr.iter().position(|x| x == max).unwrap();

    if top == 0 || arr.len() == 2 {
        return 0;
    }

    #[derive(Debug)]
    struct Value {
        value: i32,
    }
    let mut right = Value { value: *max };
    let mut left = Value { value: *max };

    println!("top: {}, left: {:?}, right: {:?}", top, left, right);

    let mut count = 1;
    for i in 1..arr.iter().len() {
        if left.value > arr[(top - i)] && right.value > arr[(top + i)] {
            count += 2;
            left.value = arr[(top - i)];
            right.value = arr[(top + i)];
            if left.value == 0 || right.value == 0 {
                break;
            }
        } else {
            break;
        }
    }
    count
}
