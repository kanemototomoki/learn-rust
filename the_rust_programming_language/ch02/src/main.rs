use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    println!("数を当ててごらん!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        print!("数を入力してね!: ");
        io::stdout().flush().unwrap();
        let mut guess = "".to_string();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み取りに失敗しました。");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}
