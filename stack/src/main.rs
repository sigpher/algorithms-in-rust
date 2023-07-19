use stack::stack::divide_by_two::{base_converter, divide_by_two};
use stack::stack::infix_to_postfix::infix_to_postfix;
use stack::stack::par_checker::par_checker;
use stack::stack::postfix_eval::postfix_evel;
use stack::Stack;

fn main() {
    basic();
    peek();
    iter();
    par_check();
    basc_convert();
    fix();
}

fn basic() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("size: {}, {:?}", s.len(), s);
    println!("pop: {:?}, size: {}", s.pop().unwrap(), s.len());
    println!("empty: {}, {:?}", s.is_empty(), s);
    s.clear();
    println!("{:?}", s);
}

fn peek() {
    let mut s = Stack::new();

    s.push(1);
    s.push(2);
    s.push(3);
    println!("{:?}", s);

    let peek_mut = s.peek_mut();
    if let Some(top) = peek_mut {
        *top = 4;
    }
    println!("top: {:?}", s.peek().unwrap());

    println!("{:?}", s);
}

fn iter() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    let sum1 = s.iter().sum::<i32>();
    let mut addend = 0;
    for item in s.iter_mut() {
        *item += 1;
        addend += 1;
    }
    let sum2 = s.iter().sum::<i32>();
    println!("{sum1} + {addend} = {sum2}");
    assert_eq!(9, s.into_iter().sum::<i32>());
}

fn par_check() {
    let text = "(5 * (3 + 2)){[]}";
    let flag = par_checker(text);
    println!("{flag}");
}

fn basc_convert() {
    let bin = divide_by_two(10);
    println!("10 is b{bin}");

    let hex = base_converter(43, 16);
    println!("43 is x{hex}");
}
fn fix() {
    let infix = "( A + B ) * C - ( D + E ) / ( F + G )";
    let postfix = infix_to_postfix(infix).unwrap();
    println!("postfix of infix `{infix}` is \n{postfix}");

    let postfix = "1 2 + 1 2 + *";

    let res = postfix_evel(postfix).unwrap();
    println!("the result of postfix {postfix} is {}", res);

    let infix = "( 10 + 2 ) * 5 + ( 6 - 2 )";

    println!(
        "the result of {infix} is: \n{}",
        postfix_evel(&infix_to_postfix(infix).unwrap()).unwrap()
    )
}
