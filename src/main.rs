fn main() {
    let v1 = vec![4, 9, 5];
    let v2 = vec![9, 4, 9, 8, 4];
    let v3 = intersect(v1, v2);
    assert_eq!(v3, vec![9, 4])
}

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut freq = vec![0; 1001];
    for key in 0..nums1.len() {
        freq[nums1[key as usize] as usize] += 1;
    }
    let mut ans = Vec::new();
    for i in 0..nums2.len() {
        if freq[nums2[i] as usize] > 0  {
            freq[nums2[i] as usize] -=1;
            ans.push(nums2[i]);
        }
    }
    return ans;
}
