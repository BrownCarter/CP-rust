fn main() {
    let i1 = str_str(String::from("hello"), String::from("ll"));
    println!("{}", i1);
}

fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() == 0 {
        return 0;
    }
    let check = haystack.find(&needle);
    match check {
       Some(check) => return check as i32,
       None => return -1,
    }
}
