#[derive(Debug)]
pub struct Deque<T> {
    pub cap: usize,
    pub data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_full(&self) -> bool {
        self.cap == self.len()
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap)
    }

    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space available".to_string());
        }
        self.data.push(val);

        Ok(())
    }

    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.data.pop()
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.data.remove(0))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { deque: self.data }
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { deque: Vec::new() };
        for item in self.data.iter() {
            iterator.deque.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { deque: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.deque.push(item);
        }
        iterator
    }
}

pub struct IntoIter<T> {
    pub deque: Vec<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.deque.is_empty() {
            None
        } else {
            Some(self.deque.remove(0))
        }
    }
}

pub struct Iter<'a, T: 'a> {
    pub deque: Vec<&'a T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.deque.is_empty() {
            None
        } else {
            Some(self.deque.remove(0))
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    pub deque: Vec<&'a mut T>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.deque.is_empty() {
            None
        } else {
            Some(self.deque.remove(0))
        }
    }
}
