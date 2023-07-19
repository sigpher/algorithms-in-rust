#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
        // Self::len(&self)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap)
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.data.pop()
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { queue: self.data }
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { queue: Vec::new() };
        for item in self.data.iter() {
            iterator.queue.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { queue: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.queue.push(item)
        }
        iterator
    }
}

pub struct IntoIter<T> {
    pub queue: Vec<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: 'a> {
    pub queue: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    pub queue: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}
