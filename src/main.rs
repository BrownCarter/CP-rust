use std::collections::HashMap;

fn main() {
    let v1 = vec![1,1,1,3,3,4,3,2,4,2];
    let check = contains_duplicate(v1);
}
fn contains_duplicate(nums: Vec<i32>) -> bool {
     let mut freq = HashMap::new();
     *freq.entry(nums).or_insert(0) + 1;
     println!("{:#?}", freq);
     return true;
}

