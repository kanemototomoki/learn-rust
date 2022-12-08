use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    let mut count = 1;
    for line in contents.lines() {
        println!("line {}: {}", count, line);
        let vec: Vec<&str> = line.split("    ").collect();
        println!("vec: {:?}", vec);
        count += 1;
    }
}
