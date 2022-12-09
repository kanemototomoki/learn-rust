fn main() {
    const MAX_POINTS: u32 = 100_000;
    let mut x = 5_u32;
    println!("The value of x is: {}", MAX_POINTS + x);
    x = 6;
    println!("The value of x is: {}", x);

    // ブロックも式なので変数宣言で使える
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let arr = [0; 5];

    // 0から4まで昇順
    for i in 0..arr.len() {
        println!("i: {}", i);
    }

    // 4から0に降順
    for i in (0..arr.len()).rev() {
        println!("rev i: {}", i);
    }
}
