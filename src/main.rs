use std::collections::HashMap;

fn main() {
    let v1 = vec![1, 1];
    let check = contains_duplicate(v1);
    println!("{}", check);
}
fn contains_duplicate(nums: Vec<i32>) -> bool {
 let mut freq = HashMap::new();
 for value in nums.iter(){
     *freq.entry(value).or_insert(0) += 1;
 }
 for (_key, value) in freq.iter() {
     if value >= &2 {
         return false;
     }
 }
 return true;
}

