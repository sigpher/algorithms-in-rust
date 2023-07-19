use deque::{deque::deque::Deque, palindrome_checker::palindrome_checker};

fn main() {
    basic();
    iter();
    palindrome();
}

fn basic() {
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_rear(3);
    let _r4 = d.add_rear(4);

    if let Err(error) = d.add_front(5) {
        println!("add_front error: {error}");
    }

    println!("{:?}", d);
    match d.remove_rear() {
        Some(data) => println!("remove rear data {data}"),
        None => println!("empty deque"),
    }

    match d.remove_front() {
        Some(data) => println!("remove front data {data}"),
        None => println!("empty deque"),
    }

    println!("empty: {}, len: {}", d.is_empty(), d.len());
    println!("full :{}, {:?}", d.is_full(), d);

    d.clear();
    println!("{:?}", d);
}

fn iter() {
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_rear(3);
    let _r4 = d.add_rear(4);

    let sum1 = d.iter().sum::<i32>();
    let mut addend = 0;

    for item in d.iter_mut() {
        *item += 1;
        addend += 1;
    }
    let sum2 = d.iter().sum::<i32>();

    println!("{sum1} + {addend} = {sum2}");
    assert_eq!(14, d.into_iter().sum::<i32>());
}

fn palindrome() {
    let pal = "rustsur";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");

    let pal = "panda";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");
}
