use std::collections::VecDeque;

fn main() {
    assert!(is_valid("({})".to_string()));
}

fn is_valid(s: String) -> bool {
    let mut container = VecDeque::new();
    for paren in s.chars(){
        match paren {
            '(' | '{' | '[' => {
                container.push_back(paren);
            },
            ')' => {
                if container.is_empty() || container.back().cloned().unwrap() != '(' {
                    return false;
                }else {
                    container.pop_back();
                }
            },
            ']' => {
                if container.is_empty() || container.back().cloned().unwrap() != '[' {
                    return false;
                }else {
                    container.pop_back();
                }
            },
            '}' => {
                if container.is_empty() || container.back().cloned().unwrap() != '{' {
                    return false;
                }else {
                    container.pop_back();
                }
            },
            _ => return false,
        }
    }
    println!("{:?}", container);
    return container.is_empty();
}
