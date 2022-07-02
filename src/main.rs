use std::cmp::max;

fn main() {
    let v1 = vec![3, 1];
    let v2 = vec![1];
    let check = max_area(5, 4, v1, v2);
    println!("{}", check);
}

fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut new_hcut = horizontal_cuts;
    let mut new_vcut = vertical_cuts;
    new_vcut.sort();
    new_hcut.sort();
    let mut best1 = 0;
    let mut best2 = 0;
    for value in 1..new_hcut.len(){
        best1 = max(best1, new_hcut[value] - new_hcut[value - 1]);
    }
    for value in 1..new_vcut.len(){
        best2 = max(best1, new_vcut[value] - new_vcut[value - 1]);
    }
    return best1 * best2;
}
