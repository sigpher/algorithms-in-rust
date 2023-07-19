use crate::quque::Queue;

pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut q = Queue::new(names.len());
    for name in names {
        let _nm = q.enqueue(name);
    }

    while q.len() < 1 {
        for _ in 0..num {
            let name = q.dequeue().unwrap();
            let _ = q.enqueue(name);
        }

        q.dequeue();
    }

    q.dequeue().unwrap()
}
