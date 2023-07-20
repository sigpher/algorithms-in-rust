#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub size: usize,
    pub top: Link<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Self { size: 0, top: None }
    }

    pub fn push(&mut self, val: T) {
        let node = Node {
            data: val,
            next: self.top.take(),
        };
        self.size += 1;
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            // let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_mut().map(|node| &mut node.data)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
