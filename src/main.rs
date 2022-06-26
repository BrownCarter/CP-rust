fn main() {
    let v1 = vec![9,8,7,6,5,4,3,2,1,0];
    let v2 = plus_one(v1);
    println!("{:?}", v2);
}

fn plus_one(digits: Vec<u64>) -> Vec<u64> {
    // TODO handle integer arithemetic overflow for i32,u32
    let s1 = digits.len() as u64 - 1;
    let mut l1 = u64::pow(10, s1.try_into().unwrap());
    let mut sum = 0;
    for i in 0..digits.len() {
        sum += digits[i] * l1;
        l1 = l1 / 10;
    }
    sum = sum + 1;
    let mut v1: Vec<u64> = Vec::new();
    while sum > 0 {
        let rem = sum % 10;
        sum = sum / 10;
        v1.push(rem);
    }
    return v1.into_iter()
        .rev()
        .collect();
}
