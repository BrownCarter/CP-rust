fn main() {
    let mut v1 = vec![0, 1, 0];
    move_zeroes(&mut v1);
    println!("{:?}", v1);
}

#[allow(unused)]
fn move_zeroes(nums: &mut Vec<i32>) {
    let mut slow: usize = 0;
    for fast in 0..nums.len() {
        if nums[fast] != 0 {
           nums.swap(fast, slow); 
           slow+=1;
        }
    }
}

