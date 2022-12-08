fn main() {
    let s = "xywrrmp".to_string();
    let t = "xywrrmu#p".to_string();
    let ans = backspace_compare(s, t);
    println!("ans: {:?}", ans);
}

pub fn backspace_compare(s: String, t: String) -> bool {
    let backspace = "#";
    let mut ans_s = "".to_string();
    let mut ans_t = "".to_string();
    let len = std::cmp::max(s.len(), t.len());

    for i in 0..len {
        let _s = s.chars().nth(i);
        let _t = t.chars().nth(i);

        if _s.is_some() && _s.unwrap().to_string() != backspace {
            ans_s.push(_s.unwrap());
        } else if !_s.is_none() {
            ans_s.pop();
        }

        if _t.is_some() && _t.unwrap().to_string() != backspace {
            ans_t.push(_t.unwrap());
        } else if !_t.is_none() {
            ans_t.pop();
        }
    }

    ans_s == ans_t
}
