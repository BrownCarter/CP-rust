fn main() {

}
#[allow(unused)]
fn length_of_last_word(s: String) -> i32 {
    let s1: Vec<&str> = s.trim()
        .split(" ")
        .collect();
    println!("{:?}", s1);
    let ans: i32 = s1[s1.len() - 1].len() as i32;
    return ans;
}
