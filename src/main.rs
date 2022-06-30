use std::collections::HashMap;

fn main() {
    let x = String::from("aabb");
    let y = first_uniq_char(x);
    println!("{}", y);
}

fn first_uniq_char(s: String) -> i32 {
    let mut freq = HashMap::new();
    for value in s.chars() {
        *freq.entry(value).or_insert(0) += 1;
    }

    for (key, value) in s.chars().enumerate() {
        let check: i32 = *freq.get(&value).unwrap();
        if check == 1 {
            return key as i32;
        }
    }
    return -1;
}
