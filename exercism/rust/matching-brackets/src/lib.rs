pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            v if v == ')' || v == '}' || v == ']' => {
                if stack.pop().unwrap_or_default() != c {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.len() == 0
}
