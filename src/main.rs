fn main() {
    let v1 = vec![vec![1, 2], vec![3, 4]];
    let v2 = matrix_reshape(v1, 2, 4);
    println!("{:?}", v2);
}

fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let mut v1: Vec<Vec<i32>> = vec![vec![0; c as usize]; r as usize];
    let total = mat[0].len() * mat.len();
    if r * c != total as i32 {
        return mat;
    }
    let c = c as usize;
    for row in 0..mat.len() * mat[0].len() {
        let n = mat[0].len();
        v1[row/c][row%c] = mat[row / n][row % n];
    }
    return v1;
}
