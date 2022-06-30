fn main() {
    assert_eq!(missing_number(vec![0, 1, 3]), 2);
    assert_eq!(missing_number(vec![0, 1, 2, 4]), 3);
    assert_eq!(missing_number(vec![]), 0);
    assert_eq!(missing_number(vec![0, 1]), 2);
}

fn missing_number(nums: Vec<i32>) -> i32 {
    let mut freq: Vec<i32> = vec![0; nums.len() + 1];
    for key in 0..nums.len() {
        freq[nums[key] as usize] += 1;
    }

    for key in 0..freq.len() {
        if freq[key] == 0 {
            return key as i32;
        }
    }
    return -1;
}
