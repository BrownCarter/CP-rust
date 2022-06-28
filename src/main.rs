use std::ptr::swap;

fn main() {
    let mut v1 = vec![0, 1, 0, 3, 12, 0, 1, 8, 7, 0];
    move_zeroes(&mut v1);
    println!("{:?}", v1);
}

fn move_zeroes(nums: &mut Vec<i32>) {
    if nums.len() == 0 {
        return;
    }
    let mut slow: usize = 0;
    for fast in 1..nums.len() {
        if nums[fast] != 0 && nums[slow] == 0 {
            // TODO learn to do this without using unsafe.
            unsafe {
                swap(&mut nums[slow], &mut nums[fast]);
            }
            slow += 1;
        }
    }
}
