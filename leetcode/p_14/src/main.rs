fn main() {
    let v = vec!["flower", "flow", "flight"];
    let v = v.clone().iter().map(|&v| v.to_string()).collect();
    let ans = longest_common_prefix(v);

    println!("ans: {}", ans);
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let first = &strs[0];
    for i in (0..first.len()).rev() {
        let prefix = &first[0..=i];
        if strs.iter().all(|s| s.starts_with(prefix)) {
            return prefix.to_string();
        }
    }
    "".to_string()
}
