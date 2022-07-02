fn main() {
    let v1 = vec![];
    let index = search(v1, -5);
    println!("{}", index);
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i8 = 0;
    let mut right: i8 = nums.len() as i8 - 1;
    while left <= right {
        let mid = (left + right) >> 1;
        if nums[mid as usize] == target {
            return mid as i32;
        }
        if nums[mid as usize] < target {
            left = mid + 1;
        } 
        if nums[mid as usize] > target {
            right = mid - 1;
        }
    }
    return -1;
}
