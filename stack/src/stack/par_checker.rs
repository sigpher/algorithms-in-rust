use crate::Stack;

pub fn par_checker(par: &str) -> bool {
    let mut stack = Stack::new();
    for ch in par.chars() {
        if ch == '(' || ch == '[' || ch == '{' {
            stack.push(ch);
        } else if ch == ')' || ch == ']' || ch == '}' {
            if stack.is_empty() {
                return false;
            }
            let close = stack.pop().unwrap();
            if !par_match(close, ch) {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closes = ")]}";

    opens.find(open) == closes.find(close)
}
