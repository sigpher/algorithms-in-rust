use std::collections::HashMap;

use crate::Stack;

use super::par_checker::par_checker;

pub fn infix_to_postfix(infix: &str) -> Option<String> {
    if !par_checker(infix) {
        return None;
    }

    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();

    for token in infix.split_whitespace() {
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if token == "(" {
            op_stack.push(token);
        } else if token == ")" {
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            while !op_stack.is_empty() && (prec[op_stack.peek().unwrap()] >= prec[token]) {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token)
        }
    }

    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }

    let mut postfix_str = String::new();

    for c in postfix {
        postfix_str += c.to_string().as_str();
        postfix_str += " ";
    }

    Some(postfix_str)
}
