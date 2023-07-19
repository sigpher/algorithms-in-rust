#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, val: T) {
        self.size += 1;
        self.data.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.data.get(self.size - 1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { stack: self.data }
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };

        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

pub struct IntoIter<T> {
    pub stack: Vec<T>,
}

pub struct Iter<'a, T: 'a> {
    pub stack: Vec<&'a T>,
}

pub struct IterMut<'a, T: 'a> {
    pub stack: Vec<&'a mut T>,
}

// implementations of 3 iterators
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
