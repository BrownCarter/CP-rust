fn main() {
    let mut v1 = vec![0,0,1,1,1,2,2,3,3,4];
    let v2 = remove_duplicates(&mut v1);
    println!("{}", v2);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut slow = 0;
    for fast in 1..nums.len() {
        if nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }
    return slow as i32 + 1;
}
