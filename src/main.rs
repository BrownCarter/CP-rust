fn main() {
    assert_eq!(generate(2), vec![vec![1], vec![1, 1]]);
}
fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut v1: Vec<Vec<i32>> = Vec::from(vec![vec![1], vec![1, 1]]);
    if num_rows == 1 {
        return  vec![v1[0].clone()];
    }
    if num_rows == 2 {
        return vec![v1[0].clone(), v1[1].clone()];
    }
    for i in 3..num_rows as usize + 1{
        let mut v2 = vec![0; i];
        let n = &v2.len();
        v2[0] = 1;
        v2[*n - 1] = 1;
        for j in 1..*n - 1 {
            v2[j] = v1[i - 2][j] + v1[i - 2][j - 1];
        }
        v1.push(v2);
    }
    return v1;
}
