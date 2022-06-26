use std::cmp::max;

fn main() {
    let v1: Vec<i32> = vec![5, 4, -1, 7, 8];
    let v2 = max_sub_array(v1);
    println!("{}", v2);
}

fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum: i32 = 0;
    let mut best: i32 = i32::MIN;
    for s in &nums {
        max_sum = max(*s, max_sum + *s);
        best = max(max_sum, best);
    }
    return best;
}
