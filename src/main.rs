fn main() {
    let mut v1 = vec![0,1,2,2,3,0,4,2];
    let v2 = remove_element(&mut v1, 2);
    println!("{}", v2);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
   let mut slow = 0;
   for fast in 0..nums.len() {
       if nums[fast] != val {
           nums[slow] = nums[fast];
           slow+=1;
       }
   }
   return slow as i32;
}
