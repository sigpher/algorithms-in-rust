type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

pub struct List<T> {
    pub size: usize,
    pub head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, elem: T) {
        let node = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));

        self.head = node;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // self.head.as_deref_mut().map(|node| &mut node.elem)
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            list: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            list: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T> {
    pub list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    pub list: Option<&'a Node<T>>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.map(|node| {
            self.list = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    pub list: Option<&'a mut Node<T>>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.take().map(|node| {
            self.list = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take()
        }
    }
}
