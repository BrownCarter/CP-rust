fn main() {
    let v1 = is_palindrome(1221);
    println!("{}", v1);
}

fn is_palindrome(x: i32) -> bool {
    let mut v1 = x;
    if x < 0 {
        return false;
    }
    let mut reversed = 0;
    while v1 > 0 {
        let rem = v1 % 10;
        reversed = reversed * 10 + rem;
        v1 = v1 / 10;
    }
    return x == reversed;
}
