use crate::Stack;

pub fn divide_by_two(mut dec_num: u32) -> String {
    let mut stack = Stack::new();
    while dec_num > 0 {
        let rem = dec_num % 2;
        stack.push(rem);
        dec_num /= 2;
    }
    let mut bin_str = "".to_string();
    while !stack.is_empty() {
        let rem = stack.pop().unwrap().to_string();
        bin_str += &rem;
    }
    bin_str
}

pub fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut stack = Stack::new();
    while dec_num > 0 {
        let rem = dec_num % base;
        stack.push(rem);
        dec_num /= base;
    }

    let mut base_str = "".to_string();
    while !stack.is_empty() {
        let rem = stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}
