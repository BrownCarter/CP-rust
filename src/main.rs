use std::cmp::max;

fn main() {
    let v1 = vec![2];
    let v2 = vec![2];
    let check = max_area(1000000000, 1000000000, v1, v2);
    println!("{}", check);
}

fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    if horizontal_cuts.len() == 1 && vertical_cuts.len() == 1 {
        return horizontal_cuts[0] * vertical_cuts[0];
    }

    let mut new_hcut = horizontal_cuts;
    let mut new_vcut = vertical_cuts;
    new_vcut.sort();
    new_hcut.sort();
    let mut best1 = 0;
    let mut best2 = 0;
    if new_hcut.len() == 1 {
        best1 = h - new_hcut[0];
    } else {
        for value in 1..new_hcut.len() {
            best1 = max(best1, new_hcut[value] - new_hcut[value - 1]);
        }
    }
    if new_vcut.len() == 1 {
        best2 = w - new_vcut[0];
    } else {
        for value in 1..new_vcut.len() {
            best2 = max(best1, new_vcut[value] - new_vcut[value - 1]);
        }
    }
    return best1 * best2;
}
