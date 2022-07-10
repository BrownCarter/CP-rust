use std::collections::HashMap;

fn main() {
    assert!(can_construct("aabb".to_string(), "aab".to_string()));
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut freq = HashMap::new();
    for value in magazine.chars() {
        *freq.entry(value).or_insert(0) += 1;
    }
    for value in ransom_note.chars() {
        if freq.get(&value).unwrap_or(&0) > &i32::MIN {
            *freq.entry(value).or_insert(0) -= 1;
        }
    }
    println!("{:?}", freq);
    let mut check = false;
    for value in ransom_note.chars() {
        if freq.get(&value).unwrap() >= &0  {
            check = true;
        }
        else {
            check = false;
            break;
        }
    }
    check
}
