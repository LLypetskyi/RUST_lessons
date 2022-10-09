use std::collections::HashMap;

fn main() {
    let s = "Learn rust rust with me me me".to_lowercase();
    let mut count_map = HashMap::new();

    for word in s.split_ascii_whitespace() {
        let count = count_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", count_map);
}
