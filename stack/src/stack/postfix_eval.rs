use crate::Stack;

pub fn postfix_evel(postfix: &str) -> Option<i32> {
    if postfix.len() < 5 {
        return None;
    }

    let mut op_stack = Stack::new();

    for token in postfix.split_whitespace() {
        if "0" <= token && token <= "9" {
            op_stack.push(token.parse::<i32>().unwrap());
        } else {
            let op2 = op_stack.pop().unwrap();
            let op1 = op_stack.pop().unwrap();
            let res = do_calc(token, op1, op2);
            op_stack.push(res);
        }
    }
    op_stack.pop()
    // Some(op_stack.pop())
}

fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else {
        if 0 == op2 {
            panic!("ZeroDivsionError: Invalid operation! 0/{op2} ");
        }
        op1 / op2
    }
}
