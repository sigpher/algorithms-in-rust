use recursion::{hanoi, num2str_rec, nums_sum, nums_sum1, nums_sum2, nums_sum3};

fn main() {
    // basic();
    // num2str();
    // hanoi_test();
    tail_recursion();
}

fn basic() {
    let ret = nums_sum(&[1, 2, 3, 4]);
    println!("{ret}");
    let nums = [2, 1, 7, 4, 5];

    let sum1 = nums_sum1(&nums);
    let sum2 = nums_sum2(&nums);
    println!("{sum1}");
    println!("{sum2}");
}

fn num2str() {
    let num = 100;
    let sb = num2str_rec(num, 2);
    let so = num2str_rec(num, 8);
    let sh = num2str_rec(num, 16);

    println!("{num} is b{sb}, o{so}, x{sh}");
}

fn hanoi_test() {
    // hanoi(1, "A", "B", "C");
    // hanoi(2, "A", "B", "C");
    hanoi(10, "A", "B", "C");
    // hanoi(4, "A", "B", "C");
}

fn tail_recursion() {
    let nums = [2, 1, 7, 4, 5];
    let sum1 = nums_sum3(0, &nums);
    let sum2 = nums_sum3(0, &nums);
    println!("sum1 is {sum1}");
    println!("sum2 is {sum2}");
}
