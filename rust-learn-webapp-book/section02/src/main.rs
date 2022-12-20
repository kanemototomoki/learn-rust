use std::{collections::HashMap, fmt::{Display, self}};
mod fraction;
mod rectangle;
mod error_sample;
mod serde_sample;

fn main() {
    println!("Hello, world!");

    let mut hash_map = HashMap::from([("a", 1), ("b", 2), ("c", 3), ("d", 4)]);

    println!("hash_map: {:?}", hash_map);
    hash_map.entry("e").or_insert(5);
    println!("hash_map: {:?}", hash_map);

    // enum
    #[derive(Debug)]
    enum Color {
        Red,
        Blue,
        Green,
        Hex(String)
    }
    let red = Color::Red;
    let hex = Color::Hex("ffffff".to_string());

    println!("{:?}, {:?}", red, hex);

    // struct
    struct User {
        name: String,
        age: u32
    }

    impl Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "user: {}, age: {}", &self.name, &self.age)
        }
    }

    impl User {
        fn new(name: String, age: u32) -> Self {
            Self {
                name,
                age
            }
        }
        fn rename(&mut self, name: String) {
            self.name = name;
        }
    }

    let mut user = User::new("taka".to_string(), 15);
    user.rename("toki".to_string());
    println!("user: {}", user);

    fraction::run();
    rectangle::run();
    let result = error_sample::run();
    println!("result: {:?}", result);

    serde_sample::run();
}
