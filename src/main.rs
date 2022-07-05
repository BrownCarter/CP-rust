fn main() {
    let mut v1 = vec![1, 2, 3, 0, 0, 0];
    let mut v2 = vec![2, 5, 6];
    merge(&mut v1, 3, &mut v2, 3);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //TODO optimize from O(nlog(n)) to O(m + n) 
    for i in 0..n {
        nums1[(m + i) as usize] = nums2[i as usize];
    }
    nums1.sort();
    println!("{:?}", nums1);
}
