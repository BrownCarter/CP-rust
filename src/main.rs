fn main() {
    let v1 = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert!(search_matrix(v1, 13));
    assert!(search_matrix(vec![vec![60]], 60));
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    for row in 0..m {
        let mut left: i32 = 0;
        let mut right: i32 = matrix[row].len() as i32 - 1;
        while left <= right {
            let mid = (left + right) >> 1;
            if matrix[row as usize][mid as usize] == target {
                return true;
            }
            if matrix[row as usize][mid as usize] < target {
                left = mid + 1;
            }
            if matrix[row as usize][mid as usize] > target {
                right = mid - 1;
            }
        }
    }
    return false;
}
