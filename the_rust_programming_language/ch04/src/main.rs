// const S: &'static str = "A";
fn main() {
    // let s = "a".to_string();
    // let sc = s.clone();
    // let ss = &s;
    // println!("S : {:p}", S);
    // println!("s : {:p}", s.as_ptr());
    // println!("sc: {:p}", sc.as_ptr());
    // println!("ss: {:p}", ss);
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
