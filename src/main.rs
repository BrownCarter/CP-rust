use std::collections::HashMap;

fn main() {
    let v1 = two_sum(vec![3, 3], 6);
    println!("{:?}", v1);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut freq = HashMap::new();
    let mut ans = vec![0; 2];
    for (key, value) in nums.iter().enumerate(){
        freq.insert(target - value, key);
    }
    for (key, &value) in nums.iter().enumerate(){
        if freq.contains_key(&value) {
            let index = freq.get(&value);
            ans[0] = *index.unwrap() as i32;
            ans[1] = key as i32;
        }
    }
    ans
}
