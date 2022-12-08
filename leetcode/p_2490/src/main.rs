fn main() {
    let ans = is_circular_sentence("leetcode exercises sound delightful".to_string());
    println!("ans: {}", ans);
}

pub fn is_circular_sentence(sentence: String) -> bool {
    let words = sentence.split_ascii_whitespace();
    let mut chars = sentence.chars();

    // L1で1個目の要素と最後の要素の比較をしている
    // 以降の行でそれぞれの末尾を先頭文字があっているかを確認している
    let result = chars.clone().next().unwrap() == chars.next_back().unwrap()
        && words
            .clone()
            .zip(words.skip(1))
            .all(|(w1, w2)| w1.ends_with(w2.chars().next().unwrap()));

    result
}
