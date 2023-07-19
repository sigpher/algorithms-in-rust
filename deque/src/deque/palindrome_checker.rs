use super::deque::Deque;

pub fn palindrome_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for ch in pal.chars() {
        let _ch = d.add_front(ch);
    }

    while d.data.len() > 1 {
        if d.remove_front() != d.remove_rear() {
            return false;
        }
    }

    true
}
