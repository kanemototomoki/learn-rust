fn main() {
    let s = "words and 987".to_string();
    // let s = "   -42-1241".to_string();
    let mut result = "".to_string();

    let mut count = 0;
    for s in s.chars() {
        println!("s: {:?}", s);
        match s {
            '-' | '+' => {
                if count == 0 {
                    count += 1;
                    result.push_str(&s.to_string());
                    continue;
                }

                break;
                // if s == '-' {
                // }
            }
            i @ '0'..='9' => result.push_str(&i.to_string()),
            other => {
                if other.is_alphanumeric() {
                    break;
                }
            }
        }
    }

    println!("result: {}", result);
    println!("result: {}", result.parse::<i32>().unwrap_or(0));
}
